# Cargo.md
## Git dependency
```toml
// The version will go into the lock file, can update with `cargo update rand`
[dependencies]
rand = { git = "https://github.com/rust-lang-nursery/rand.git"}

// with rev, not recommended
rand = { git = "https://github.com/rust-lang-nursery/rand.git", rev="9f35b8e"}
```

### Make smaller binary
```toml
// This will stop cleanup when the program panics, leaving it to the OS
[profile.release]
panic = 'abort'
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

