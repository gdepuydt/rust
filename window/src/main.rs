// Remove later
#![allow(unused_imports)]
//

#[cfg(target_os="windows")]
#[path="cwin.rs"]
mod os;

#[cfg(target_os="linux")]
#[path="clin.rs"]
mod os;

use os::*;

fn main() {
    println!("Hello, world!");
}
