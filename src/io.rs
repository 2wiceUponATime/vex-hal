// SPDX-FileCopyrightText: 2023 Gavin Niederman
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

mod stdio;

#[doc(inline)]
pub use no_std_io::io::*;
pub use stdio::{
    __print, Stdin, StdinLock, Stdout, StdoutLock, dbg, print, println, stdin, stdout,
};
