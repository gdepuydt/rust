#![no_std]
#![no_main]


use core::panic::PanicInfo;


/// This function is called o panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop{}
}
