[CI](./ci.md)


## Commands
- `cargo build [--release | --debug]`
- `cargo test` Run through all test cases 
- `cargo test foo` Run through a single test case
- `cargo doc --open` Create documentation and serve locally
- `cargo run` Compiles debug build and runs it
- `cargo clean` Removes target directories
- `cargo update` Update all dependencies
- `cargo update -p rand` Update a single crate

## External commands
Install these external commands annotating the command name with `cargo-` e.g. `cargo install cargo-watch`
- `cargo watch -x "check"` Run a cargo command every time some code changes
- `cargo expand` Generate the output of macro code generation

## Git dependency
```toml
// The version will go into the lock file, can update with `cargo update rand`
[dependencies]
rand = { git = "https://github.com/rust-lang-nursery/rand.git"}

// with rev, not recommended
rand = { git = "https://github.com/rust-lang-nursery/rand.git", rev="9f35b8e"}
```

## Cargo Home
defaults to `$HOME/.cargo/` but can be altered with the environment variable `CARGO_HOME`. `crates.io/crates/home` will allow you to access the directory from rust code.

### Files
- `config.toml` global config.
- `credentials.toml` stores `cargo login` information
- `crates.toml` information about installed crates
- `crates2.json` information about installed crates in JSON

### Directories
- `bin` executables installed via `cargo install`
- `git/db` bare repo from crate dependencies (.git folder)
- `git/checkouts` source code for each commit being used checked out to this folder
- `registry/index` bare .git repo with metadata
- `registry/cache` downloaded dependencies, gzip files with .crate extension
- `registry/src` contains source code from the dependencies

### Required cache folders for CI
- `bin/`
- `registry/index/`
- `registry/cache/`
- `git/db/`

### Make smaller binary
```toml
// This will stop cleanup when the program panics, leaving it to the OS
[profile.release]
panic = 'abort'
```

## Publish to crates.io
### Cargo.toml minimum requirements 
```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2018"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"

[dependencies]
```
### Publishing
First create an account at [crates.io](https://crates.io)

Then create an API key in the [me page](https://crates.io/me)

And run commands:
```bash
cargo login [api_key]
cargo publish
```

## Add a feature from a crate
```toml
syn = { version = "1.0", features = ["extra-traits"] }
```

## Change the default build target
Make a new file in the project root at:
`.cargo/config`
```toml
[build]
target = "wasm32-unknown-unknown"
```
This will also update how rust analyzer reacts to
```
#[cfg(target_arch = "wasm32")]
```

## Common Package Layout

```text
├── Cargo.lock
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── main.rs
│   └── bin/
│       ├── named-executable.rs
│       ├── another-executable.rs
│       └── multi-file-executable/
│           ├── main.rs
│           └── some_module.rs
├── benches/
│   ├── large-input.rs
│   └── multi-file-bench/
│       ├── main.rs
│       └── bench_module.rs
├── examples/
│   ├── simple.rs
│   └── multi-file-example/
│       ├── main.rs
│       └── ex_module.rs
└── tests/
    ├── some-integration-tests.rs
    └── multi-file-test/
        ├── main.rs
        └── test_module.rs
```