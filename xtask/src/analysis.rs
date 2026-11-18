// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    ffi::c_uint,
    fmt::Write,
    iter,
    path::Path,
};

use capstone::{
    Capstone, RegId,
    arch::{
        BuildsCapstone, DetailsArchInsn,
        arm::{
            ArchMode::{Arm, Thumb},
            ArmCC, ArmInsn, ArmOperand, ArmOperandType, ArmReg,
        },
    },
};
use futures::{StreamExt, TryStreamExt, stream};
use indicatif::ProgressStyle;
use indoc::{formatdoc, indoc};
use itertools::Itertools;
use memchr::memmem;
use object::{Object, ObjectSection, ObjectSymbol, SymbolKind};
use regex::Regex;
use tokio::{fs, io, process::Command};

use crate::{Context, Error, Platform, from, latest_downloaded_os, latest_downloaded_sdk, run};

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum Entry {
    #[default]
    Present,
    Null,
    Unimplemented,
}

type Table = [Entry; 0x400];

pub async fn extract_jump_table(ctx: Context<'_>) -> Result<(), Error> {
    let table = if ctx.platform == Platform::V5 {
        [Entry::Present; 0x400]
    } else {
        read_table(&ctx).await?
    };

    let obj_path = ctx.root.join(format!(
        "decomp/{}/libs/{}_apijump.c.{}",
        ctx.platform,
        ctx.platform.lower(),
        ctx.platform.archive_extension()
    ));

    let include = latest_downloaded_sdk(&ctx).await?;
    let header =
        fs::read_to_string(include.join(format!("vex{}/include/{0}_api.h", ctx.platform.lower())))
            .await?;
    let known_header = fs::read_to_string(ctx.root.join("decomp/known_signatures.h")).await?;

    let mut signatures = HashMap::new();
    let sig_re = Regex::new(r"(?ui)^([a-z\d_]+ +)(\**)(([a-z_\d]+) *\(.*\) *; *)$").unwrap();
    let mut padding = None;

    for (line, needs_padding) in header
        .lines()
        .zip(iter::repeat(false))
        .chain(known_header.lines().zip(iter::repeat(true)))
    {
        if line.starts_with("#") || line.starts_with("typedef ") {
            continue;
        }
        if let Some(caps) = sig_re.captures(line) {
            if padding.is_none() {
                padding = Some(caps[1].len() + caps[2].len());
            }
            signatures.entry(caps[4].to_owned()).or_insert_with(|| {
                if needs_padding {
                    format!(
                        "{:<w$}{}{}",
                        &caps[1],
                        &caps[2],
                        &caps[3],
                        w = padding.unwrap() - caps[2].len()
                    )
                } else {
                    line.to_owned()
                }
            });
        }
    }

    ctx.progress
        .set_message(format!("Reading {}", obj_path.display()));
    if let Err(e) = process(
        &ctx,
        &include,
        fs::read(&obj_path).await?,
        &signatures,
        &table,
        padding.unwrap(),
    )
    .await
    {
        return Err(match e {
            AnalysisError::Object(e) => Error::Object(obj_path, e),
            AnalysisError::Capstone(e) => Error::Capsone(obj_path, e),
            AnalysisError::Io(e) => Error::Io(e),
        });
    }
    ctx.progress.finish_with_message("Done");
    Ok(())
}

pub async fn disassemble_jump_table(ctx: Context<'_>) -> Result<(), Error> {
    if ctx.platform == Platform::V5 {
        return Err(Error::Static("not available for V5"));
    }

    let cs_arm = Capstone::new()
        .arm()
        .mode(Arm)
        .detail(true)
        .build()
        .unwrap();
    let cs_thumb = Capstone::new()
        .arm()
        .mode(Thumb)
        .detail(true)
        .build()
        .unwrap();

    let latest = latest_downloaded_os(&ctx).await?;
    let boot_path = latest.join("BOOT.bin");
    let boot = fs::read(&boot_path).await?;
    let assets = fs::read(latest.join("assets.bin")).await?;
    let boot_start = ctx.platform.boot_load_address();

    // Which entries are real, implemented functions (not null or the stub).
    let table = read_table(&ctx).await?;

    // Entry offset -> API name, inverted from the SDK's apijump wrappers: each
    // wrapper calls the entry that is its own OS implementation.
    ctx.progress.set_message("recovering names");
    let obj_path = ctx.root.join(format!(
        "decomp/{}/libs/{}_apijump.c.{}",
        ctx.platform,
        ctx.platform.lower(),
        ctx.platform.archive_extension()
    ));
    let functions = analyze(&ctx, &fs::read(&obj_path).await?).map_err(|e| match e {
        AnalysisError::Object(e) => Error::Object(obj_path.clone(), e),
        AnalysisError::Capstone(e) => Error::Capsone(obj_path.clone(), e),
        AnalysisError::Io(e) => Error::Io(e),
    })?;
    let mut names = HashMap::new();
    for func in functions {
        for call in func.calls {
            names.entry(call).or_insert_with(|| func.name.clone());
        }
    }

    // Wrap BOOT.bin in an ELF placed at its load address, carrying a symbol for
    // every implemented entry so objdump can name branch targets, plus `$a`/`$t`
    // mapping symbols so it decodes each function in the right instruction set.
    let mut args: Vec<String> = ["-I", "binary", "-O", "elf32-littlearm", "-B", "arm"]
        .map(String::from)
        .to_vec();
    args.push("--set-section-flags".into());
    args.push(".data=alloc,load,readonly,code,contents".into());
    args.push("--change-section-address".into());
    args.push(format!(".data={boot_start:#x}"));
    args.push("--set-start".into());
    args.push(format!("{boot_start:#x}"));

    // (entry offset, start address, stop address) for each distinct function.
    let mut dumps = Vec::new();
    let mut seen = HashSet::new();
    for (i, &entry) in assets[0..0x1000].as_chunks::<4>().0.iter().enumerate() {
        if table[i] != Entry::Present {
            continue;
        }
        let entry = u32::from_le_bytes(entry);
        let ptr = entry & !1;
        let thumb = entry & 1 == 1;
        if ptr < boot_start || (ptr - boot_start) as usize >= boot.len() {
            ctx.progress
                .println(format!("warning: entry 0x{:03x} points outside BOOT.bin", i * 4));
            continue;
        }
        let offset = (ptr - boot_start) as u64;
        let entry_offset = (i * 4) as u32;
        let name = names
            .get(&entry_offset)
            .cloned()
            .unwrap_or_else(|| format!("fn_{entry_offset:03x}"));

        // The function symbol carries the Thumb bit in its value so objdump
        // resolves `blx`/`bx` targets to it.
        args.push("--add-symbol".into());
        args.push(format!(
            "{name}=.data:{:#x},function,global",
            offset | thumb as u64
        ));

        // One mapping symbol and one disassembly range per distinct function;
        // several entries may alias the same implementation.
        if seen.insert(offset) {
            args.push("--add-symbol".into());
            args.push(format!("{}=.data:{offset:#x}", if thumb { "$t" } else { "$a" }));

            let size = function_size(
                if thumb { &cs_thumb } else { &cs_arm },
                &boot,
                offset,
                thumb,
            )
            .map_err(|e| Error::Capsone(boot_path.clone(), e))?;
            let start = boot_start as u64 + offset;
            dumps.push((entry_offset, start, start + size));
        }
    }

    let elf_path = ctx.root.join(format!("decomp/{}/os.elf", ctx.platform));
    args.push(boot_path.to_string_lossy().into_owned());
    args.push(elf_path.to_string_lossy().into_owned());

    ctx.progress.set_message("building symbol table");
    run(
        Command::new("arm-none-eabi-objcopy").args(&args),
        elf_path.with_extension("objcopy.log"),
    )
    .await?;

    ctx.progress.set_length(dumps.len() as u64);
    ctx.progress.set_position(0);
    ctx.progress.set_style(
        ProgressStyle::with_template("{spinner} {prefix:>4} [{pos}/{len}] {msg}").unwrap(),
    );
    ctx.progress.set_message("disassembling");

    let jobs = std::thread::available_parallelism().map_or(4, |n| n.get());
    let mut outputs: Vec<(u32, Vec<u8>)> = stream::iter(dumps)
        .map(|(entry_offset, start, stop)| {
            let elf_path = &elf_path;
            let progress = ctx.progress;
            async move {
                let dump = run(
                    Command::new("arm-none-eabi-objdump")
                        .args(["-d", "-C"])
                        .arg(format!("--start-address={start:#x}"))
                        .arg(format!("--stop-address={stop:#x}"))
                        .arg(elf_path),
                    elf_path.with_extension("objdump.log"),
                )
                .await?;
                progress.inc(1);
                Ok::<_, Error>((entry_offset, dump))
            }
        })
        .buffered(jobs)
        .try_collect()
        .await?;

    let mut out = Vec::new();
    for (entry_offset, dump) in outputs.drain(..) {
        out.extend_from_slice(format!("\n// 0x{entry_offset:03x}\n").as_bytes());
        out.extend_from_slice(&dump);
    }
    fs::write(ctx.root.join(format!("decomp/{}/jump.s", ctx.platform)), out).await?;

    ctx.progress.finish_with_message("Done");
    Ok(())
}

fn find_iter_single(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let mut it = memmem::find_iter(haystack, needle);
    match (it.next(), it.next()) {
        (Some(offset), None) => Some(offset),
        _ => None,
    }
}

async fn read_table(ctx: &Context<'_>) -> Result<Table, Error> {
    let latest = match latest_downloaded_os(ctx).await {
        Ok(latest) => latest,
        Err(Error::NoOsFound) => {
            ctx.progress.println(format!(
                "warning: no downloaded {} OS to read jump table from",
                ctx.platform,
            ));
            return Ok([Entry::Present; 0x400]);
        }
        Err(e) => return Err(e),
    };

    let boot = fs::read(latest.join("BOOT.bin")).await?;

    let Some(offset) = find_iter_single(&boot, b"bad function called\n") else {
        return Err(Error::Static(
            r"expected exactly one instance of 'bad function called\n' in OS binary",
        ));
    };
    let string_addr = (ctx.platform.boot_load_address() + offset as u32).to_le_bytes();

    let Some(offset) = find_iter_single(&boot, &string_addr) else {
        return Err(Error::Static(
            r"expected exactly one pointer to 'bad function called\n' in OS binary",
        ));
    };
    let stub = (ctx.platform.boot_load_address() + offset as u32 - 7).to_le_bytes();

    let assets = fs::read(latest.join("assets.bin")).await?;
    let mut table = [Entry::Present; 0x400];
    for (i, &entry) in assets[0..0x1000].as_chunks::<4>().0.iter().enumerate() {
        if entry == [0; 4] {
            table[i] = Entry::Null;
        }
        if entry == stub {
            table[i] = Entry::Unimplemented;
        }
    }

    Ok(table)
}

#[derive(Debug)]
enum AnalysisError {
    Object(object::Error),
    Capstone(capstone::Error),
    Io(io::Error),
}

from!(
    AnalysisError,
    |e: object::Error| Self::Object(e),
    |e: capstone::Error| Self::Capstone(e),
    |e: io::Error| Self::Io(e),
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tracked {
    Address(u32),
    Entry(u32),
}

#[derive(Debug, Default, Clone)]
struct Registers([Option<Tracked>; 15]);

impl Registers {
    fn idx(r: RegId) -> Option<usize> {
        Some(match r.0 as c_uint {
            ArmReg::ARM_REG_R0 => 0,
            ArmReg::ARM_REG_R1 => 1,
            ArmReg::ARM_REG_R2 => 2,
            ArmReg::ARM_REG_R3 => 3,
            ArmReg::ARM_REG_R4 => 4,
            ArmReg::ARM_REG_R5 => 5,
            ArmReg::ARM_REG_R6 => 6,
            ArmReg::ARM_REG_R7 => 7,
            ArmReg::ARM_REG_R8 => 8,
            ArmReg::ARM_REG_R9 => 9,
            ArmReg::ARM_REG_R10 => 10,
            ArmReg::ARM_REG_R11 => 11,
            ArmReg::ARM_REG_R12 => 12,
            ArmReg::ARM_REG_SP => 13,
            ArmReg::ARM_REG_LR => 14,
            _ => return None,
        })
    }

    fn set(&mut self, r: RegId, value: Option<Tracked>) {
        let Some(idx) = Self::idx(r) else {
            return;
        };
        self.0[idx] = value;
    }

    fn get(&self, r: RegId) -> Option<Tracked> {
        self.0[Self::idx(r)?]
    }

    /// Keeps only the values both states agree on.
    fn merge(&mut self, other: &Self) {
        for (a, b) in self.0.iter_mut().zip(&other.0) {
            if a != b {
                *a = None;
            }
        }
    }
}

/// Determines the size in bytes of the function beginning at `start`, an offset
/// into `data`, when no symbol table records its length.
///
/// This is a recursive-descent sweep: it follows control flow from the entry,
/// extending the function's extent to cover every instruction it can reach,
/// every in-range branch target, and every PC-relative literal pool slot it
/// loads from, stopping each path at a terminator (an unconditional branch,
/// `bx`, or a `pop`/`ldm`/`ldr`/`mov` into `pc`). The returned length spans up
/// to the highest such address, so it includes a trailing literal pool.
///
/// `thumb` selects the literal-pool alignment rule and must match the mode of
/// `cs`. Branch targets are followed only when they land at or after `start`
/// and within `data`. A `b` tail-call into a neighbouring function would still
/// be followed, so the result is an upper bound when functions are not laid out
/// contiguously.
fn function_size(
    cs: &Capstone,
    data: &[u8],
    start: u64,
    thumb: bool,
) -> Result<u64, capstone::Error> {
    let mut worklist = vec![start];
    let mut visited = HashSet::new();
    let mut end = start;

    while let Some(mut addr) = worklist.pop() {
        while (addr as usize) < data.len() && visited.insert(addr) {
            let insns = cs.disasm_count(&data[addr as usize..], addr, 1)?;
            let Some(insn) = insns.iter().next() else {
                break;
            };
            let detail = cs.insn_detail(insn)?;
            let arch = detail.arch_detail();
            let arm = arch.arm().unwrap();
            let ops = arm.operands().collect_vec();
            let id = ArmInsn::from(insn.id().0);
            let next = insn.address() + insn.bytes().len() as u64;
            end = end.max(next);

            // A PC-relative load pulls from a literal pool, usually sited after
            // the code; its slot extends the function's byte footprint.
            if id == ArmInsn::ARM_INS_LDR
                && let [
                    _,
                    ArmOperand {
                        op_type: ArmOperandType::Mem(mem),
                        ..
                    },
                ] = ops.as_slice()
                && mem.base().0 == ArmReg::ARM_REG_PC as u16
            {
                let disp = if ops[1].subtracted {
                    -(mem.disp() as i64)
                } else {
                    mem.disp() as i64
                };
                let lit_addr = if thumb {
                    ((insn.address() + 4) & !3) as i64 + disp
                } else {
                    (insn.address() + 8) as i64 + disp
                } as u64;
                end = end.max(lit_addr + 4);
            }

            // Queue the destination of any direct branch for its own sweep.
            if let (
                ArmInsn::ARM_INS_B | ArmInsn::ARM_INS_CBZ | ArmInsn::ARM_INS_CBNZ,
                Some(ArmOperand {
                    op_type: ArmOperandType::Imm(target),
                    ..
                }),
            ) = (id, ops.last())
            {
                let target = *target as u32 as u64;
                if start <= target && (target as usize) < data.len() {
                    worklist.push(target);
                }
            }

            // Stop this path when control cannot fall through to `next`.
            let unconditional = matches!(arm.cc(), ArmCC::ARM_CC_AL | ArmCC::ARM_CC_INVALID);
            let writes_pc = ops.iter().any(|op| {
                matches!(op.op_type, ArmOperandType::Reg(r) if r.0 == ArmReg::ARM_REG_PC as u16)
            });
            let terminates = unconditional
                && match id {
                    ArmInsn::ARM_INS_B | ArmInsn::ARM_INS_BX => true,
                    ArmInsn::ARM_INS_POP
                    | ArmInsn::ARM_INS_LDM
                    | ArmInsn::ARM_INS_LDR
                    | ArmInsn::ARM_INS_MOV => writes_pc,
                    _ => false,
                };
            if terminates {
                break;
            }
            addr = next;
        }
    }

    Ok(end - start)
}

/// One jump-table wrapper recovered from an SDK archive object: its symbol name
/// and the jump-table entry offsets it dispatches through (empty when it calls
/// none).
struct Function {
    name: String,
    calls: Vec<u32>,
}

/// Disassembles every text symbol in an SDK archive object and recovers, for
/// each, the jump-table entries it calls. This is the structured core shared by
/// jump-table consumers; rendering (signatures, annotations) is left to the
/// caller.
fn analyze(ctx: &Context<'_>, bytes: &[u8]) -> Result<Vec<Function>, AnalysisError> {
    let file = object::File::parse(bytes)?;

    let cs_arm = Capstone::new()
        .arm()
        .mode(Arm)
        .detail(true)
        .build()
        .unwrap();
    let cs_thumb = Capstone::new()
        .arm()
        .mode(Thumb)
        .detail(true)
        .build()
        .unwrap();

    let symbols = file
        .symbols()
        .filter(|s| s.kind() == SymbolKind::Text)
        .collect_vec();

    ctx.progress.set_length(symbols.len() as u64);
    ctx.progress.set_style(
        ProgressStyle::with_template("{spinner} {prefix:>4} [{pos}/{len}] {msg}").unwrap(),
    );

    let mut functions = Vec::with_capacity(symbols.len());
    for sym in symbols {
        let Some(idx) = sym.section_index() else {
            continue;
        };
        let section = file.section_by_index(idx)?;
        let data = section.data()?;
        let name = sym.name()?;
        let mut calls = vec![];

        let thumb = sym.address() & 1 == 1;
        let start = (sym.address() & !1) - section.address();
        let bytes = &data[start as usize..][..sym.size() as usize];

        let cs = if thumb { &cs_thumb } else { &cs_arm };
        let end = start + sym.size();
        let mut regs = Registers::default();
        // Register state saved at the targets of forward branches.
        let mut pending: HashMap<u64, Registers> = HashMap::new();
        // Whether the previous instruction can fall through to this one.
        let mut reachable = true;
        for insn in cs.disasm_all(bytes, start)?.iter() {
            let detail = cs.insn_detail(insn)?;
            let arch = detail.arch_detail();
            let arm = arch.arm().unwrap();
            let ops = arm.operands().collect_vec();
            let id = ArmInsn::from(insn.id().0);

            if let Some(saved) = pending.remove(&insn.address()) {
                if reachable {
                    regs.merge(&saved);
                } else {
                    regs = saved;
                }
            } else if !reachable {
                regs = Registers::default();
            }

            match (id, ops.as_slice()) {
                (
                    ArmInsn::ARM_INS_LDR,
                    [
                        ArmOperand {
                            op_type: ArmOperandType::Reg(rd),
                            ..
                        },
                        ArmOperand {
                            op_type: ArmOperandType::Mem(mem),
                            ..
                        },
                    ],
                ) => {
                    let disp = if ops[1].subtracted {
                        -(mem.disp() as i64)
                    } else {
                        mem.disp() as i64
                    };
                    let reg = regs.get(mem.base());
                    regs.set(*rd, None);
                    if let Some(Tracked::Address(base)) = reg {
                        regs.set(*rd, Some(Tracked::Entry((base as i64 + disp) as u32)));
                    } else if mem.base().0 == ArmReg::ARM_REG_PC as u16 {
                        let lit_addr = if thumb {
                            ((insn.address() + 4) & !3) as i64 + disp
                        } else {
                            (insn.address() + 8) as i64 + disp
                        } as u64;
                        let literal = u32::from_le_bytes(
                            data[lit_addr as usize..lit_addr as usize + 4]
                                .try_into()
                                .unwrap(),
                        );
                        if ctx.platform.jump_table_start() <= literal
                            && literal < ctx.platform.jump_table_start() + 0x1000
                        {
                            regs.set(*rd, Some(Tracked::Address(literal)));
                        }
                    }
                }
                (
                    ArmInsn::ARM_INS_BX | ArmInsn::ARM_INS_BLX,
                    [
                        ArmOperand {
                            op_type: ArmOperandType::Reg(rd),
                            ..
                        },
                    ],
                ) if let Some(Tracked::Entry(ptr)) = regs.get(*rd) => {
                    calls.push(ptr - ctx.platform.jump_table_start());
                }
                (
                    ArmInsn::ARM_INS_MOV,
                    [
                        ArmOperand {
                            op_type: ArmOperandType::Reg(rd),
                            ..
                        },
                        ArmOperand {
                            op_type: ArmOperandType::Reg(rm),
                            ..
                        },
                    ],
                ) => {
                    regs.set(*rd, regs.get(*rm));
                }
                _ => {
                    for &r in detail.regs_write() {
                        regs.set(r, None);
                    }
                }
            }

            if let (
                ArmInsn::ARM_INS_B | ArmInsn::ARM_INS_CBZ | ArmInsn::ARM_INS_CBNZ,
                Some(ArmOperand {
                    op_type: ArmOperandType::Imm(target),
                    ..
                }),
            ) = (id, ops.last())
            {
                let target = *target as u32 as u64;
                if insn.address() < target && target < end {
                    pending
                        .entry(target)
                        .and_modify(|r| r.merge(&regs))
                        .or_insert_with(|| regs.clone());
                }
            }

            let unconditional = matches!(arm.cc(), ArmCC::ARM_CC_AL | ArmCC::ARM_CC_INVALID);
            let writes_pc = ops.iter().any(|op| {
                matches!(op.op_type, ArmOperandType::Reg(r) if r.0 == ArmReg::ARM_REG_PC as u16)
            });
            reachable = !(unconditional
                && match id {
                    ArmInsn::ARM_INS_B | ArmInsn::ARM_INS_BX => true,
                    ArmInsn::ARM_INS_POP
                    | ArmInsn::ARM_INS_LDM
                    | ArmInsn::ARM_INS_LDR
                    | ArmInsn::ARM_INS_MOV => writes_pc,
                    _ => false,
                });
        }

        functions.push(Function {
            name: name.to_owned(),
            calls,
        });
        ctx.progress.inc(1);
    }

    Ok(functions)
}

async fn process(
    ctx: &Context<'_>,
    sdk_path: &Path,
    bytes: Vec<u8>,
    signatures: &HashMap<String, String>,
    table: &Table,
    padding: usize,
) -> Result<(), AnalysisError> {
    let functions = analyze(ctx, &bytes)?;

    let mut out = formatdoc! {r#"
        #include "sdk/{}/vex{}/include/{1}_api.h"

        typedef uint32_t unknown;
        typedef int (*rmTask_t)(void);

    "#, sdk_path.file_name().unwrap().to_str().unwrap(),
    ctx.platform.lower()}
    .to_owned();

    // Scheduler types used by known_signatures.h. The IQ2 SDK already declares these.
    if ctx.platform != Platform::Iq2 {
        out.push_str(indoc! {"
            typedef void (*rmTaskSimple_t)(void);
            typedef int (*rmTaskArg_t)(void *);

            typedef enum {
              kTaskUndefined = 0,
              kTaskInitializing,
              kTaskRunning,
              kTaskTimerWait,
              kTaskStop,
              kTaskSemaphoreWait,
              kTaskSemaphoreRunPending,
              kTaskSuspended,
              kTaskInitializeSuspended,
              kTaskRunningSuspended,
              kTaskTimerWaitSuspended,
              kTaskSemaphoreWaitSuspended,
              kTaskComplete = 99
            } rmTaskStates;

            typedef struct {
              uint8_t nOwningTask;
              uint8_t nLockCount;
            } rmSemaphore;

        "});
    }

    for Function { name, calls } in &functions {
        let sig = match signatures.get(name) {
            Some(sig) => Cow::Borrowed(sig),
            None => Cow::Owned(format!("{:<padding$}{name}(/* unknown */);", "unknown ",)),
        };

        write!(out, "{sig}").unwrap();

        if calls.is_empty() {
            out.push('\n');
        } else {
            writeln!(
                out,
                " // {}",
                calls.iter().format_with(", ", |&value, f| f(&format_args!(
                    "0x{value:03x}{}",
                    match table[value as usize / 4] {
                        Entry::Present => "",
                        Entry::Null => " (null)",
                        Entry::Unimplemented => " (unimplemented)",
                    },
                )))
            )
            .unwrap();
        }
    }

    fs::write(
        ctx.root.join(format!("decomp/{}/jump.h", ctx.platform)),
        out,
    )
    .await?;

    Ok(())
}
