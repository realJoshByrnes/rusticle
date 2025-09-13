#![no_std]
#![no_main]

use core::{ffi::c_void, panic::PanicInfo, ptr::null_mut};

// Import Win32 API items (from kernel32.lib / kernel32.dll)
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

// This is a special constant which can be passed to some Win32 API's to get the standard output handle
const STD_OUTPUT_HANDLE: u32 = -11i32 as u32;

// By moving the HELLO_WORLD_MESSAGE to .text, we save 4 bytes on i686
#[unsafe(link_section = ".text")]
// This is the actual message we'll print to the console
static HELLO_WORLD_MESSAGE: [u8; 14] = *b"Hello, world!\n";

#[unsafe(no_mangle)]
// mainCRTStartup should return `!`, but Windows allows us to return a `u32` to exit the current thread and provide an exit code
pub extern "C" fn mainCRTStartup() -> u32 {
    unsafe {
        // Write our HELLO_WORLD_MESSAGE to the console
        WriteFile(
            STD_OUTPUT_HANDLE as _,
            HELLO_WORLD_MESSAGE.as_ptr(),
            HELLO_WORLD_MESSAGE.len() as u32,
            null_mut(),
            null_mut(),
        );
        // Returning 0 (SUCCESS) is equivalent to calling ExitProcess(0) if only one thread is running.
        0
    }
}

// Even though we don't use a panic handler, rust still requires one (dummy implementation)
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
