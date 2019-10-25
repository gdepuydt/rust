use libc::write;
use std::ffi::c_void;
use std::mem::transmute;

fn main() {
    
    #[cfg(target_os = "linux")]
    let string = "Hello From Linux!\n";
    let len = string.len();
    
    unsafe {
        write(0, transmute::<*const u8, *const c_void>(string.as_ptr()), len)
    };
    
    #[cfg(target_os = "windows")]
    println!("Hello From Windows!");

    

}
