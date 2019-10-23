#![feature(lang_items, core_intrinsics)]
#![feature(compiler_builtins_lib)]
//#![feature(start)]
#![no_std]
#![no_main]

extern crate compiler_builtins;
use core::intrinsics;
use core::panic::PanicInfo;


// Pull in the system libc library for what crt0.o likely requires.
extern crate libc;

#[no_mangle] // ensure that this symbol is called `main` in the output
pub extern fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    1
}


// These functions are used by the compiler, but not
// for a bare-bones hello world. These are normally
// provided by libstd.
#[lang = "eh_personality"]
#[no_mangle]
pub extern fn rust_eh_personality() {
}

// This function may be needed based on the compilation target.
#[lang = "eh_unwind_resume"]
#[no_mangle]
pub extern fn rust_eh_unwind_resume() {
}

#[lang = "panic_impl"]
#[no_mangle]
pub extern fn rust_begin_panic(_info: &PanicInfo) -> ! {
    unsafe { intrinsics::abort() }
}