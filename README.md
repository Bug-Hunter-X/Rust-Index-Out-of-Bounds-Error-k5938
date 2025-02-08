# Rust Index Out of Bounds Example

This repository demonstrates a common error in Rust: the `index out of bounds` error. The code attempts to access elements in a vector using both indexing and the `get()` method.  One of the access attempts results in a runtime panic because the index is out of bounds.

The `bug.rs` file shows the erroneous code. The `bugSolution.rs` file provides a corrected version demonstrating safe access to vector elements using `get()` and handling potential errors with `match`.