// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

extern crate alloc;

use alloc::collections::BinaryHeap;
use core::task::Waker;

use crate::time::Instant;

pub(crate) struct Sleeper {
    pub deadline: Instant,
    pub waker: Waker,
}

impl PartialEq for Sleeper {
    fn eq(&self, other: &Self) -> bool {
        self.deadline.eq(&other.deadline)
    }
}
impl PartialOrd for Sleeper {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Sleeper {}
impl Ord for Sleeper {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        // NOTE: Sleeper with the earliest deadline (minimum `Instant` ordering) must have the
        // highest priority in the queue.
        self.deadline.cmp(&other.deadline).reverse()
    }
}

#[derive(Default)]
pub struct Reactor {
    pub(crate) sleepers: BinaryHeap<Sleeper>,
}

impl Reactor {
    pub const fn new() -> Self {
        Self {
            sleepers: BinaryHeap::new(),
        }
    }

    /// Queues any sleepers ready to wake. Returns the time until a sleeper will awaken, or zero if
    /// one is ready.
    pub fn tick(&mut self) {
        let now = Instant::now();

        while let Some(next) = self.sleepers.peek() {
            let time_to_wake = next.deadline.duration_since(now);

            if time_to_wake.is_zero() {
                // We want to wake all of the expired sleepers, so don't stop early.

                let sleeper = self.sleepers.pop().unwrap();
                sleeper.waker.wake();
            } else {
                // Since we've popped all the expired sleepers, we now just care about how long we
                // have to wait until the next one. The queue is drained in order,
                // so we surely have encountered the smallest TTW in the list already.

                break;
            }
        }
    }
}
