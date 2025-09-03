#![no_std]
#![no_main]

use core::{ffi::c_void, panic::PanicInfo, ptr::null_mut};

unsafe extern "system" {
    fn GetStdHandle(nStdHandle: u32) -> *mut c_void;
    fn WriteConsoleA(
        hConsoleOutput: *mut c_void,
        lpBuffer: *const u8,
        nNumberOfCharsToWrite: u32,
        lpNumberOfCharsWritten: *mut u32,
        lpReserved: *mut c_void,
    ) -> i32;
    fn ExitProcess(uExitCode: u32) -> !;
}

const STD_OUTPUT_HANDLE: u32 = -11i32 as u32;

#[unsafe(no_mangle)]
pub extern "C" fn mainCRTStartup() -> ! {
    let msg = b"Hello, world!\n";
    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        let mut written = 0;
        WriteConsoleA(
            handle,
            msg.as_ptr(),
            msg.len() as u32,
            &mut written,
            null_mut(),
        );
        ExitProcess(0)
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
