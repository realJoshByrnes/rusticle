#![no_main]

use std::process::exit;

#[unsafe(no_mangle)]
pub extern "C" fn mainCRTStartup() -> ! {
    println!("Hello, world!");
    exit(0)
}
