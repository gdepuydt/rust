#![no_std]
#![no_main]
#![feature(asm)]


use core::panic::PanicInfo;


/// This function is called o panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn main() -> ! {
    loop{}
}


#[no_mangle]
pub extern "C" fn _start() {
    unsafe {
        // Align the stack to the 16 byte boundary as the stdlib C preamble would
        // not strictly necessary for this to work
        asm!(
            "
                and $$0xfffffffffffffff0, %rsp;
                call main;
            "
        );
    }
}





