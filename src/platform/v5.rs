pub use vex_sdk_jumptable::*;

pub fn exit() -> ! {
    unsafe { vexSystemExitRequest() };
    loop {}
}

pub fn yield_now() {
    unsafe { vexTasksRun() };
}
