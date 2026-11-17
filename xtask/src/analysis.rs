// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

use std::{borrow::Cow, collections::HashMap, ffi::c_uint, fmt::Write, iter, path::Path};

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
use indicatif::ProgressStyle;
use indoc::{formatdoc, indoc};
use itertools::Itertools;
use object::{Object, ObjectSection, ObjectSymbol, SymbolKind};
use regex::Regex;
use tokio::{fs, io};

use crate::{Context, Error, Platform, from, latest_downloaded_sdk};

pub async fn extract_jump_table(ctx: Context<'_>) -> Result<(), Error> {
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

async fn process(
    ctx: &Context<'_>,
    sdk_path: &Path,
    bytes: Vec<u8>,
    signatures: &HashMap<String, String>,
    padding: usize,
) -> Result<(), AnalysisError> {
    let file = object::File::parse(&*bytes)?;

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

    let symbols = file
        .symbols()
        .filter(|s| s.kind() == SymbolKind::Text)
        .collect_vec();

    ctx.progress.set_length(symbols.len() as u64);
    ctx.progress.set_style(
        ProgressStyle::with_template("{spinner} {prefix:>4} [{pos}/{len}] {msg}").unwrap(),
    );

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
                calls
                    .iter()
                    .format_with(", ", |value, f| f(&format_args!("0x{value:03x}")))
            )
            .unwrap();
        }
        ctx.progress.inc(1);
    }

    fs::write(
        ctx.root.join(format!("decomp/{}/jump.h", ctx.platform)),
        out,
    )
    .await?;

    Ok(())
}
