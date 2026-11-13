use std::{
    fmt::{self, Display},
    io::{self, Cursor},
    path::PathBuf,
    time::Duration,
};

use clap::{Parser, Subcommand, ValueEnum};
use futures::future::join_all;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use serde::Deserialize;
use zip::ZipArchive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum Platform {
    Exp,
    Iq2,
    V5,
}

impl Platform {
    pub const fn as_path_part(self) -> &'static str {
        match self {
            Self::Exp => "EXP",
            Self::Iq2 => "IQ2",
            Self::V5 => "V5",
        }
    }

    pub fn sdk_base_url(self) -> impl Display {
        std::fmt::from_fn(move |f| {
            write!(
                f,
                "https://content.vexrobotics.com/vexos/public/{}/vscode/sdk/cpp",
                self.as_path_part()
            )
        })
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
}

#[derive(Debug, Deserialize)]
struct SdkManifest {
    latest: String,
}

macro_rules! error_enum {
    ($vis:vis enum $name:ident {
        $($variant:ident($ty:ty)),+ $(,)?
    }) => {
        $vis enum $name {
            $($variant($ty)),+
        }

        $(impl From<$ty> for $name {
            fn from(value: $ty) -> Self {
                Self::$variant(value)
            }
        })+

        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    $(Self::$variant(err) => err.fmt(f)),+
                }
            }
        }
    };
}

struct ExistsError {
    path: PathBuf,
}

impl Display for ExistsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} already exists", self.path.display())
    }
}

struct CreateDirError {
    path: PathBuf,
    inner: io::Error,
}

impl Display for CreateDirError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error creating {}: {}", self.path.display(), self.inner)
    }
}

error_enum! {
    enum DownloadSdkError {
        Reqwest(reqwest::Error),
        Zip(zip::result::ZipError),
        Io(std::io::Error),
        Exists(ExistsError),
        CreateDir(CreateDirError),
    }
}

async fn download_sdk(p: Platform, pb: &ProgressBar) -> Result<(), DownloadSdkError> {
    pb.set_message("fetching manifest");
    let url = format!("{}/manifest.json", p.sdk_base_url());
    let SdkManifest { latest } = reqwest::get(&url).await?.json().await?;

    pb.set_message(format!("downloading {latest}"));
    let url = format!("{}/{latest}.zip", p.sdk_base_url());
    let bytes = reqwest::get(&url).await?.bytes().await?;
    let mut archive = ZipArchive::new(Cursor::new(bytes))?;
    let target_dir = format!("decomp/{}/sdk/{latest}", p.as_path_part());

    pb.set_message(format!("extracting {latest}"));
    if std::fs::exists(&target_dir)? {
        return Err(ExistsError {
            path: target_dir.into(),
        }
        .into());
    }
    if let Err(e) = std::fs::create_dir_all(&target_dir) {
        return Err(CreateDirError {
            path: target_dir.into(),
            inner: e,
        }
        .into());
    }
    archive.extract(&target_dir)?;
    pb.finish_with_message(format!("done ({latest})"));
    Ok(())
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::DownloadSdk => {
            let mp = MultiProgress::new();
            let style = ProgressStyle::with_template("{spinner} {prefix:>4} {msg}").unwrap();
            join_all(as_platform_array(cli.platform).into_iter().map(|p| {
                let pb = mp.add(ProgressBar::new_spinner());
                pb.set_style(style.clone());
                pb.set_prefix(p.as_path_part());
                pb.enable_steady_tick(Duration::from_millis(100));
                async move {
                    if let Err(e) = download_sdk(p, &pb).await {
                        pb.abandon_with_message(format!("error: {e}"));
                    }
                }
            }))
            .await;
        }
    }
}
