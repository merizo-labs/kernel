#![no_std]
#![no_main]
extern crate alloc;

mod vga_buffer;

use panic_halt as _;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
