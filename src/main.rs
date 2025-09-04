#![no_std]
#![no_main]

use core::{ffi::c_void, panic::PanicInfo, ptr::null_mut};

unsafe extern "system" {
    fn WriteFile(
        hFile: *mut c_void,
        lpBuffer: *const u8,
        nNumberOfBytesToWrite: u32,
        lpNumberOfBytesWritten: *mut u32,
        lpOverlapped: *mut c_void,
    ) -> i32;
    fn ExitProcess(uExitCode: u32) -> !;
}

const STD_OUTPUT_HANDLE: u32 = -11i32 as u32;

#[unsafe(no_mangle)]
pub extern "C" fn mainCRTStartup() -> ! {
    let msg = b"Hello, world!\n";
    unsafe {
        let handle = STD_OUTPUT_HANDLE as *mut c_void;
        WriteFile(
            handle,
            msg.as_ptr(),
            msg.len() as u32,
            null_mut(),
            null_mut(),
        );
        ExitProcess(0)
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
