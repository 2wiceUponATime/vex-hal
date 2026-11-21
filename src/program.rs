// SPDX-FileCopyrightText: 2023 Gavin Niederman
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! User program state.

use bitflags::bitflags;

use crate::platform::{SIG_MAGIC, vcodesig, vexSystemLinkAddrGet};

/// Identifies the type of binary to VEXos.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u32)]
#[non_exhaustive]
pub enum ProgramType {
    /// User program binary.
    User = 0,
}

/// The owner (originator) of the user program
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u32)]
pub enum ProgramOwner {
    /// Program is a system binary.
    System = 0,

    /// Program originated from VEX.
    Vex = 1,

    /// Program originated from a partner developer.
    Partner = 2,
}

bitflags! {
    /// Program Startup Options
    ///
    /// These bitflags are part of the [`CodeSignature`] and determine some small aspects of program
    /// behavior when running under VEXos. This struct contains the flags with publicly documented
    /// behavior.
    #[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
    pub struct ProgramOptions: u32 {
        /// Inverts the background color to pure white.
        const INVERT_DEFAULT_GRAPHICS = 1 << 0;

        /// VEXos scheduler simple tasks will be killed when the program requests exit.
        const KILL_TASKS_ON_EXIT = 1 << 1;

        /// If VEXos is using the Light theme, inverts the background color to pure white.
        const THEMED_DEFAULT_GRAPHICS = 1 << 2;
    }
}

/// Program Code Signature
///
/// The first 16 bytes of a VEX user program contains a code signature header, which has some basic
/// metadata and startup flags for the program. This signature must be at the start of the binary
/// for VExos to recognize our binary as a program.
///
/// The signature is emitted by the linker script into the `.code_signature` section at the start of
/// the binary. Its fields default to a [`ProgramType::User`] / [`ProgramOwner::Partner`] program
/// with no [`ProgramOptions`], and can be overridden at link time, e.g.
/// `-C link-arg=--defsym=__vcodesig_options=0x123`. This type is used to *read back* the running
/// program's signature via [`code_signature`].
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct CodeSignature(vcodesig, [u32; 4]);

impl CodeSignature {
    /// Creates a new signature given a program type, owner, and flags.
    #[must_use]
    pub const fn new(
        program_type: ProgramType,
        owner: ProgramOwner,
        options: ProgramOptions,
    ) -> Self {
        Self(
            vcodesig {
                magic: SIG_MAGIC,
                r#type: program_type as _,
                owner: owner as _,
                options: options.bits(),
            },
            [0; 4],
        )
    }

    /// Returns the program owner specified by this signature.
    ///
    /// See [`ProgramOwner`] for more info.
    #[must_use]
    pub const fn owner(&self) -> ProgramOwner {
        match self.0.owner {
            0 => ProgramOwner::System,
            1 => ProgramOwner::Vex,
            2 => ProgramOwner::Partner,
            _ => unreachable!(),
        }
    }

    /// Returns the program type specified by this signature.
    ///
    /// See [`ProgramType`] for more info.
    #[must_use]
    pub const fn program_type(&self) -> ProgramType {
        match self.0.r#type {
            0 => ProgramType::User,
            _ => unreachable!(),
        }
    }

    /// Returns the program startup options specified by this signature.
    ///
    /// See [`ProgramOptions`] for more info.
    #[must_use]
    pub const fn options(&self) -> ProgramOptions {
        ProgramOptions::from_bits_retain(self.0.options)
    }
}

/// Returns the code signature of the currently running program.
#[inline]
#[must_use]
pub fn code_signature() -> CodeSignature {
    #[cfg(not(feature = "platform-mock"))]
    {
        unsafe extern "C" {
            // Defined in https://github.com/rust-lang/rust/blob/master/compiler/rustc_target/src/spec/targets/armv7a_vex_v5_linker_script.ld.
            static __user_ram_start: CodeSignature;
        }

        unsafe { core::ptr::read(&raw const __user_ram_start) }
    }

    #[cfg(feature = "platform-mock")]
    CodeSignature::new(
        ProgramType::User,
        ProgramOwner::Partner,
        ProgramOptions::empty(),
    )
}

/// Returns a raw pointer to the currently linked file.
///
/// If no file is linked to the current program, this function will return a null pointer.
#[inline]
#[must_use]
pub fn linked_file() -> *mut () {
    unsafe { vexSystemLinkAddrGet() as *mut () }
}
