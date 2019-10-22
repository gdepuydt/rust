# A Freestanding Rust Binary

This crate compiles without linking to the Rust Standard Libary.

There are mainly 2 ways to go about this:

1) Creating a Freerunning app, ie. without a minimal C runtime and no Rust stdlib [[1]]. there's also this [post](https://users.rust-lang.org/t/win32-no-std-no-main-no-libc/30038), which refers to this [repository](https://github.com/blaubart69/rust_win32_no_std_no_main/tree/b728a18beb9edf3f9aecc6740cc6924196095746). Can you make it work? without the winapi? its possible. but don't forget to enable the profiles for aborting on panic.

2) Creating an app without using the Rust stdlib, but that has a minimal C runtime setup [[2]].  


A minimal example of the first solution is the 'free_running' crate. The second solution is implemented as the 'no_std' crate.  

Note that RLS in VSCode will indicate errors, but these can be ignored. Compilation should succeed and the program execute.


[1]: https://os.phil-opp.com/freestanding-rust-binary/#introduction

[2]: https://doc.rust-lang.org/1.30.0/unstable-book/language-features/lang-items.html#using-libc

