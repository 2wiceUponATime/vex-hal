use super::jump_table;

/// Handle to an opaque [`V5_Device`].
#[allow(non_camel_case_types)]
pub type V5_DeviceT = *mut V5_Device;

/// A device plugged into a smart port.
#[allow(non_camel_case_types)]
pub type V5_Device = *mut core::ffi::c_void;

pub mod abs_enc;
pub mod adi;
pub mod ai_vision;
pub mod arm;
pub mod battery;
pub mod competition;
pub mod controller;
pub mod device;
pub mod display;
pub mod distance;
pub mod file;
pub mod generic_radio;
pub mod generic_serial;
pub mod gps;
pub mod imu;
pub mod led;
pub mod light_tower;
pub mod magnet;
pub mod motor;
pub mod optical;
pub mod pneumatic;
pub mod range;
pub mod serial;
pub mod system;
pub mod task;
pub mod touch;
pub mod vision;

pub use motor::*;
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
