// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

use spin::RelaxStrategy;

use crate::platform;

pub struct YieldNow;

impl RelaxStrategy for YieldNow {
    #[inline(always)]
    fn relax() {
        platform::yield_now();
    }
}

pub type Mutex<T> = spin::mutex::Mutex<T, YieldNow>;
