[CI](CI.md)

## Package Layout

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

## Commands
### build
Build crate
```bash
cargo build [--release | --debug]
```

### test
Run through all test cases defined in ./tests/
```bash
cargo test
```

Run through a single test case
```bash
cargo test foo
```

### run

### clean


### update
Update all crates
```bash
cargo update
```

Update a single crate
```bash
cargo update -p rand
```

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
```text
config.toml:      global config.
credentials.toml: stores `cargo login` information
.crates.toml:     information about installed crates
.crates2.json:    information about installed crates in JSON
```

### Directories
```text
bin:            executables installed via `cargo install`
git/db:         bare repo from crate dependencies (.git folder)
git/checkouts:  source code for each commit being used checked out to this folder
registry/index: bare .git repo with metadata
registry/cache: downloaded dependencies, gzip files with .crate extension
registry/src:   contains source code from the dependencies
```

### Required cache folders for CI
```text
bin/
registry/index/
registry/cache/
git/db/
```