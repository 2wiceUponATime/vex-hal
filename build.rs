// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

use std::{
    env,
    fmt::{Debug, Display},
    fs,
    path::PathBuf,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Platform {
    Exp,
    Iq2,
    V5,
    Mock,
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Exp => "exp",
            Self::Iq2 => "iq2",
            Self::V5 => "v5",
            Self::Mock => "mock",
        })
    }
}

impl Debug for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "platform-{self}")
    }
}

impl Platform {
    pub fn is_enabled(&self) -> bool {
        env::var_os(format!(
            "CARGO_FEATURE_PLATFORM_{}",
            self.to_string().to_uppercase()
        ))
        .is_some()
    }

    pub fn target(&self) -> &str {
        match self {
            Platform::Exp | Platform::Iq2 => "thumbv7em-none-eabihf",
            Platform::V5 => "thumbv7a-vex-v5",
            Platform::Mock => unimplemented!(),
        }
    }
}

fn main() {
    let platforms = [Platform::Exp, Platform::Iq2, Platform::V5, Platform::Mock];
    let enabled: Vec<_> = platforms.iter().filter(|p| p.is_enabled()).collect();
    let platform = match enabled.as_slice() {
        [one] => **one,
        [] => panic!("enable one of: platform-exp, platform-iq2, platform-v5, platform-mock"),
        _ => panic!("platform features are mutually exclusive, got {enabled:?}"),
    };

    if platform == Platform::Mock {
        return;
    }

    let target = env::var("TARGET").unwrap();
    let expected_target = platform.target();
    assert!(
        platform.target() == target,
        "{platform} is not supported on target {target}; use {expected_target} instead"
    );

    if platform != Platform::V5 {
        let out = PathBuf::from(env::var("OUT_DIR").unwrap());
        let link = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("link");
        fs::copy(link.join(format!("vex-{platform}.ld")), out.join("vex.ld")).unwrap();
        fs::copy(link.join("vex-common.ld"), out.join("vex-common.ld")).unwrap();

        println!("cargo:rustc-link-search={}", out.display());
        println!("cargo:rustc-link-arg=-Tvex.ld");
        println!("cargo:rerun-if-changed=link");
    }
}
