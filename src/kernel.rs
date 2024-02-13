#![no_std]
#![no_main]

#[path = "terminal.rs"]
mod terminal;

#[panic_handler]
fn panic(_:&::core::panic::PanicInfo) -> ! {
    loop{}
}

#[no_mangle]
pub unsafe extern "C" fn kernel_main() -> ! {
    let mut terminal = terminal::Terminal { row: 0, column: 0, colour: 0, buffer: 0 as *mut u16 };
    terminal.init();

    terminal.write("Hello, world!");

    loop{}
}