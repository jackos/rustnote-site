# Publishing
## Cargo.toml minimum requirements 
```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2018"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"

[dependencies]
```

## Publishing to crates.io
First create an account at [crates.io](https://crates.io)

Then create an API key in the [me page](https://crates.io/me)

And run commands:
```bash
cargo login [api_key]
cargo publish
```
