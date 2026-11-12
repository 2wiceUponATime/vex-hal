use core::alloc::Layout;

use talc::{
    TalcLock,
    base::{Talc, binning::Binning},
    source::Source,
};

use crate::thread;

unsafe extern "C" {
    static mut __heap_start: u8;
    static mut __heap_end: u8;
}

#[global_allocator]
static ALLOCATOR: TalcLock<thread::Mutex<()>, LinkerHeap> =
    TalcLock::new(LinkerHeap { claimed: false });

/// Allocator source that claims the `.heap` region reserved by the linker
/// script (`__heap_start..__heap_end`) on the first allocation. Linker symbol
/// addresses aren't usable in const context, so this can't be a
/// [`talc::source::Claim`] built in the static initializer.
#[derive(Debug)]
struct LinkerHeap {
    claimed: bool,
}

// SAFETY: `acquire` only claims the linker-reserved heap region, which nothing
// else touches.
unsafe impl Source for LinkerHeap {
    fn acquire<B: Binning>(talc: &mut Talc<Self, B>, _layout: Layout) -> Result<(), ()> {
        if talc.source.claimed {
            return Err(());
        }
        talc.source.claimed = true;

        let start = &raw mut __heap_start;
        let size = (&raw mut __heap_end).addr() - start.addr();
        // SAFETY: the linker reserves `.heap` exclusively for the allocator.
        unsafe { talc.claim(start, size) }.map(|_| ()).ok_or(())
    }
}
