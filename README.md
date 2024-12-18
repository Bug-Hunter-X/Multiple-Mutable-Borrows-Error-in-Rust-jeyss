# Multiple Mutable Borrows in Rust

This repository demonstrates a common error in Rust: creating multiple mutable borrows of the same variable.  Rust's ownership and borrowing system prevents data races and ensures memory safety, but this restriction can sometimes be tricky to work around.

The `bug.rs` file contains the erroneous code that attempts to have multiple mutable references, resulting in a compile-time error. The `bugSolution.rs` file presents a possible solution using techniques like cloning or using a mutable reference wrapped in a struct.