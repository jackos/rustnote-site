# Documentation

## Use markdown inside comments
This assert will be tested with `cargo test` wow! Keeps documentation examples in sync with the library
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

## Containing docstring
If this docstring is at the root of the crate,
it will describe the entire crate
```rust
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.
```