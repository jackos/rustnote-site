# Documentation
Cargo has inbuilt documentation tooling, also allowing tests inside the documentation so that it never becomes out of sync with the code base

### Serve documentation in http
cargo doc --open

### Use markdown inside comments
This assert will be tested with `cargo test`. Keeps documentation examples in sync with the library
```rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```
The above comment applies tested html documentation to the add_one function built from markdown. So simple, so powerful.

### Containing docstring
If this docstring is at the root of the crate,
it will describe the entire crate
```rust
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.
```
