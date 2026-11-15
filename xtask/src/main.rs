use std::{
    fmt::{self, Display},
    io::{self, Cursor},
    path::{Path, PathBuf},
    time::Duration,
};

use clap::{Parser, Subcommand, ValueEnum};
use futures::{StreamExt, TryStreamExt, future::join_all, stream};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use serde::Deserialize;
use tokio::{fs, process::Command};
use zip::ZipArchive;

use crate::analysis::extract_jump_table;

mod analysis;

macro_rules! from {
    ($enum:ty, $(|$var:ident: $ty:ty| $into:expr),+ $(,)?) => {
        $(impl From<$ty> for $enum {
            fn from($var: $ty) -> Self {
                $into
            }
        })+
    };
}
use from;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum Platform {
    Exp,
    Iq2,
    V5,
}

impl Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Exp => "EXP",
            Self::Iq2 => "IQ2",
            Self::V5 => "V5",
        })
    }
}

impl Platform {
    pub fn sdk_base_url(self) -> impl Display {
        std::fmt::from_fn(move |f| {
            write!(
                f,
                "https://content.vexrobotics.com/vexos/public/{self}/vscode/sdk/cpp",
            )
        })
    }

    pub const fn lower(self) -> &'static str {
        match self {
            Self::Exp => "exp",
            Self::Iq2 => "iq2",
            Self::V5 => "v5",
        }
    }

    pub const fn archive(self) -> &'static str {
        match self {
            Self::Exp => "libexprt.a",
            Self::Iq2 => "libviq2rt.a",
            Self::V5 => "libv5rt.a",
        }
    }

    pub const fn archive_extension(self) -> &'static str {
        match self {
            Self::Exp | Self::Iq2 => "o",
            Self::V5 => "obj",
        }
    }

    pub const fn jump_table_start(self) -> u32 {
        match self {
            Self::Exp => 0x301fc000,
            Self::Iq2 => 0x101fc000,
            Self::V5 => 0x37fc000,
        }
    }
}

fn as_platform_array(platform: Option<Platform>) -> Vec<Platform> {
    match platform {
        Some(p) => vec![p],
        None => vec![Platform::Exp, Platform::Iq2, Platform::V5],
    }
}

#[derive(Debug, Parser)]
struct Cli {
    /// Platform (defaults to all)
    #[arg(long)]
    platform: Option<Platform>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Download the latest official VEX SDK for decompilation
    DownloadSdk,
    /// Extract `lib[platform]rt.a` from the downloaded SDK
    ExtractSdkLibs,
    /// Disassemble all extracted `.o` files
    Dis,
    ExtractJumpTable,
}

struct Context<'a> {
    platform: Platform,
    progress: &'a ProgressBar,
    root: &'a Path,
}

#[derive(Debug, Deserialize)]
struct SdkManifest {
    latest: String,
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Http(reqwest::Error),
    #[error(transparent)]
    Extract(zip::result::ZipError),
    #[error(transparent)]
    Io(io::Error),
    #[error("{0} already exists")]
    Exists(PathBuf),
    #[error("error creating {0}: {1}")]
    CreateDir(PathBuf, io::Error),
    #[error("no SDK found")]
    MissingSdk,
    #[error("{exec} execution failed - see {log}")]
    Command { exec: String, log: PathBuf },
    #[error("error reading {0}: {1}")]
    Object(PathBuf, object::Error),
    #[error("error reading {0}: {1}")]
    Capsone(PathBuf, capstone::Error),
}

from!(
    Error,
    |err: reqwest::Error| Self::Http(err),
    |err: zip::result::ZipError| Self::Extract(err),
    |err: io::Error| Self::Io(err),
);

async fn run(cmd: &mut Command, log: PathBuf) -> Result<Vec<u8>, Error> {
    let output = cmd.output().await?;
    if output.status.success() {
        return Ok(output.stdout);
    }

    let contents = [
        format!(
            "command: {:?}\n{}\n\nstdout:\n",
            cmd.as_std(),
            output.status
        )
        .as_bytes(),
        &output.stdout,
        b"\nstderr:\n",
        &output.stderr,
    ]
    .concat();
    if let Some(parent) = log.parent() {
        fs::create_dir_all(parent).await?;
    }
    fs::write(&log, contents).await?;

    Err(Error::Command {
        exec: cmd.as_std().get_program().to_string_lossy().into_owned(),
        log,
    })
}

async fn download_sdk(ctx: Context<'_>) -> Result<(), Error> {
    ctx.progress.set_message("fetching manifest");
    let url = format!("{}/manifest.json", ctx.platform.sdk_base_url());
    let SdkManifest { latest } = reqwest::get(&url).await?.json().await?;

    let target_dir = ctx.root.join(format!("decomp/{}/sdk", ctx.platform));
    if fs::try_exists(target_dir.join(&latest)).await? {
        return Err(Error::Exists(target_dir));
    }

    ctx.progress.set_message(format!("downloading {latest}"));
    let url = format!("{}/{latest}.zip", ctx.platform.sdk_base_url());
    let bytes = reqwest::get(&url).await?.bytes().await?;
    let mut archive = ZipArchive::new(Cursor::new(bytes))?;

    ctx.progress.set_message(format!("extracting {latest}"));
    if let Err(e) = fs::create_dir_all(&target_dir).await {
        return Err(Error::CreateDir(target_dir, e));
    }
    archive.extract(&target_dir)?;
    ctx.progress.finish_with_message(format!("done ({latest})"));
    Ok(())
}

async fn extract_sdk_libs(ctx: Context<'_>) -> Result<(), Error> {
    ctx.progress.set_message("finding latest SDK version");
    let sdk = latest_downloaded_sdk(&ctx).await?;

    let archive = ctx.platform.archive();
    ctx.progress.set_message(format!("extracting {archive}"));
    let libs_path = ctx.root.join(format!("decomp/{}/libs", ctx.platform));
    fs::create_dir_all(&libs_path).await?;
    run(
        Command::new("arm-none-eabi-ar").args([
            "x",
            &sdk.join(format!("vex{}/{archive}", ctx.platform.lower()))
                .to_string_lossy(),
            "--output",
            &libs_path.to_string_lossy(),
        ]),
        ctx.root
            .join(format!("decomp/extract-{}.log", ctx.platform)),
    )
    .await?;
    ctx.progress
        .finish_with_message(format!("extracted {archive}"));
    Ok(())
}

async fn disassemble(ctx: Context<'_>) -> Result<(), Error> {
    let libs = ctx.root.join(format!("decomp/{}/libs", ctx.platform));
    let out_dir = ctx.root.join(format!("decomp/{}/dis", ctx.platform));
    fs::create_dir_all(&out_dir).await?;

    let mut objs = Vec::new();
    let mut entries = fs::read_dir(&libs).await?;
    while let Some(e) = entries.next_entry().await? {
        let p = e.path();
        if p.extension()
            .is_some_and(|x| x == ctx.platform.archive_extension())
        {
            objs.push(p);
        }
    }
    objs.sort();

    ctx.progress.set_length(objs.len() as u64);
    ctx.progress.set_style(
        ProgressStyle::with_template("{spinner} {prefix:>4} [{pos}/{len}] {msg}").unwrap(),
    );

    let jobs = std::thread::available_parallelism().map_or(4, |n| n.get());
    stream::iter(objs)
        .map(|obj| {
            let out_dir = &out_dir;
            async move {
                let dump = run(
                    Command::new("arm-none-eabi-objdump")
                        .args(["-d", "-r", "-C"])
                        .arg(&obj),
                    obj.with_added_extension("dis.log"),
                )
                .await?;
                let name = obj.file_stem().unwrap().to_string_lossy().into_owned();
                fs::write(out_dir.join(format!("{name}.s")), dump).await?;
                Ok::<_, Error>(name)
            }
        })
        .buffer_unordered(jobs)
        .try_for_each(|name| {
            ctx.progress.inc(1);
            ctx.progress.set_message(name);
            async { Ok(()) }
        })
        .await?;

    ctx.progress.finish_with_message("done");
    Ok(())
}

async fn latest_downloaded_sdk(ctx: &Context<'_>) -> Result<PathBuf, Error> {
    let base = ctx.root.join(format!("decomp/{}/sdk", ctx.platform));
    let mut entries = fs::read_dir(&base).await?;
    let mut latest = None;
    while let Some(entry) = entries.next_entry().await.ok().flatten() {
        if entry.file_type().await.is_ok_and(|t| t.is_dir())
            && let Ok(name) = entry.file_name().into_string()
        {
            latest = latest.max(Some(name));
        }
    }
    latest.ok_or(Error::MissingSdk).map(|v| base.join(v))
}

#[tokio::main]
async fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let cli = Cli::parse();
    let mp = MultiProgress::new();
    let style = ProgressStyle::with_template("{spinner} {prefix:>4} {msg}").unwrap();
    join_all(
        as_platform_array(cli.platform)
            .into_iter()
            .map(async |platform| {
                let pb = mp.add(ProgressBar::new_spinner());
                pb.set_style(style.clone());
                pb.set_prefix(platform.to_string());
                pb.enable_steady_tick(Duration::from_millis(100));
                let ctx = Context {
                    platform,
                    progress: &pb,
                    root,
                };
                if let Err(e) = match cli.command {
                    Commands::DownloadSdk => download_sdk(ctx).await,
                    Commands::ExtractSdkLibs => extract_sdk_libs(ctx).await,
                    Commands::Dis => disassemble(ctx).await,
                    Commands::ExtractJumpTable => extract_jump_table(ctx).await,
                } {
                    pb.abandon_with_message(format!("error: {e}"));
                }
            }),
    )
    .await;
}
