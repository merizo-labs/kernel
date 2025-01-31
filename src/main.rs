#![no_std]
#![no_main]

mod vga_buffer;

use panic_halt as _;
use spin::Mutex;
use talc::{ClaimOnOom, Span, Talc};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    Span::empty();
    loop {}
}
