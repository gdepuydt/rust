# A Freestanding Rust Binary

This crate compiles without linking to the Rust Standard Libary.

There are mainly 2 ways to go about this:

1) Creating a Freerunning app, ie. without a minimal C runtime and no Rust stdlib [[1]].

2) Creating an app without using the Rust stdlib, but that has a minimal C runtime setup [[2]][[3]].  

Reference links:

[1]: https://os.phil-opp.com/freestanding-rust-binary/#introduction

[3]: https://doc.rust-lang.org/1.30.0/unstable-book/language-features/lang-items.html#using-libc

[2]: https://doc.rust-lang.org/1.16.0/book/no-stdlib.html