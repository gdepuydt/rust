# A Freestanding Rust Binary

This crate compiles without linking to the Rust Standard Libary.

There are mainly 2 ways to go about this:

1) Creating a Freerunning app, ie. without a minimal C runtime and no Rust stdlib [[1]].

2) Creating an app without using the Rust stdlib, but that has a minimal C runtime setup [[2]].  


A minimal example of the first solution is the 'free_running' crate. The second solution is implemented as the 'no_std' crate.  

Note that RLS in VSCode will indicate errors, but these can be ignored. Compilation should succeed and the program execute.

> In many cases, you may need to manually link to the compiler_builtins crate when building a no_std binary. You may observe this via linker error messages such as "undefined reference to `__rust_probestack'". Using this crate also requires enabling the library feature compiler_builtins_lib. You can read more about this [here](https://doc.rust-lang.org/1.30.0/unstable-book/library-features/compiler-builtins-lib.html).

[1]: https://os.phil-opp.com/freestanding-rust-binary/#introduction

[2]: https://doc.rust-lang.org/1.30.0/unstable-book/language-features/lang-items.html#using-libc

