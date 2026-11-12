use super::jump_table;

pub mod serial;
pub mod system;
pub mod task;

pub use serial::*;
pub use system::*;
pub use task::*;

pub const JUMP_TABLE_START: usize = 0x301fc000;

pub fn exit() -> ! {
    unsafe { vexSystemExitRequest() };
    // Keep yielding to VEXos while it tears us down.
    loop {
        unsafe { vexTaskSleep(1000) }
    }
}
