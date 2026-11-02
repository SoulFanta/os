#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Функция panic будет вызвана при возникновении паники в программе
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
// Не используем стандартную библиотеку, поэтому нужно определить точку входа вручную
// Функция _start будет вызвана при запуске программы
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}