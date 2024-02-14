#![no_std]
#![no_main]

#![allow(warnings)]

#[path = "terminal.rs"]
mod terminal;

use terminal::Terminal;

use core::arch::asm;

#[panic_handler]
fn panic(_:&::core::panic::PanicInfo) -> ! {
    loop{}
}

#[no_mangle]
pub unsafe extern "C" fn kernel_main() -> ! {
    let mut terminal: Terminal = Default::default();
    terminal.init();

    asm!("nop");

    terminal.write("Hello, world!");

    loop{}
}