#![no_std]
#![no_main]

use core::{ffi::c_void, panic::PanicInfo, ptr::null_mut};

#[link(name = "kernel32")]
unsafe extern "system" {
    fn WriteFile(
        hFile: *mut c_void,
        lpBuffer: *const u8,
        nNumberOfBytesToWrite: u32,
        lpNumberOfBytesWritten: *mut u32,
        lpOverlapped: *mut c_void,
    ) -> i32;
}

const STD_OUTPUT_HANDLE: u32 = -11i32 as u32;

#[unsafe(link_section = ".text")]
pub static HELLO_WORLD_MESSAGE: [u8; 14] = *b"Hello, world!\n";

#[unsafe(no_mangle)]
pub extern "C" fn mainCRTStartup() -> u32 {
    unsafe {
        let handle = STD_OUTPUT_HANDLE as *mut c_void;
        WriteFile(
            handle,
            HELLO_WORLD_MESSAGE.as_ptr(),
            HELLO_WORLD_MESSAGE.len() as u32,
            null_mut(),
            null_mut(),
        );
        // Returning 0 from mainCRTStartup is equivalent to calling ExitProcess(0) if only one thread is running.
        0
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
