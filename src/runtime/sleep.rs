// SPDX-FileCopyrightText: 2023 Gavin Niederman
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! Utilities for tracking time.
//!
//! This module provides a types for executing code after a set period of time.
//!
//! - [`Sleep`] is a future that does no work and completes at a specific [`Instant`] in time.
//! - [`sleep`] and [`sleep_until`] provide ways to yield control away from a future for or until a
//!   specific instant in time.

use core::{
    pin::Pin,
    task::{Context, Poll, Waker},
    time::Duration,
};

use crate::{
    runtime::{executor::EXECUTOR, reactor::Sleeper},
    time::Instant,
};

#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Sleep {
    deadline: Instant,
    registered_waker: Option<Waker>,
}

impl Future for Sleep {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.deadline {
            return Poll::Ready(());
        }

        // Register a waker on the reactor to only poll this future when the deadline passes.
        //
        // We should only push to the sleeper queue if we either haven't pushed
        // (`self.registered.waker == None`) or if !w.will_wake(cx.waker()), meaning the already
        // registered waker will not wake up the same task as the current waker indicating that the
        // sleep has potentially been moved across executors.
        if self
            .registered_waker
            .as_ref()
            .is_none_or(|w| !w.will_wake(cx.waker()))
        {
            let this = self.get_mut();
            this.registered_waker = Some(cx.waker().clone());

            EXECUTOR.with_reactor(|reactor| {
                reactor.sleepers.push(Sleeper {
                    deadline: this.deadline,
                    waker: cx.waker().clone(),
                });
            });
        }

        Poll::Pending
    }
}

/// Waits until `duration` has elapsed.
///
/// This function returns a future that will complete after the given duration, effectively yielding
/// the current task for a period of time.
///
/// Equivalent to `sleep_until(Instant::now() + duration)`.
pub fn sleep(duration: Duration) -> Sleep {
    Sleep {
        deadline: Instant::now() + duration,
        registered_waker: None,
    }
}

/// Waits until `deadline` is reached.
///
/// This function returns a future that will complete once a given `Instant` in time has been
/// reached.
pub const fn sleep_until(deadline: Instant) -> Sleep {
    Sleep {
        deadline,
        registered_waker: None,
    }
}
