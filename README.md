# Rust Unwrap Panic Example

This repository demonstrates a common error in Rust: panicking due to using `unwrap()` on an `Option` that may be `None`.  Specifically, the example shows how accessing a vector element using `get()` and then using `unwrap()` can cause a panic if the vector is empty or the index is out of bounds.

The `bug.rs` file contains the buggy code. The `bugSolution.rs` file shows how to fix the problem using pattern matching or the `expect()` method for more descriptive error handling.  See the comments for explanations.