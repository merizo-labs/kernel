#![no_std]
#![no_main]
use panic_halt as _;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
