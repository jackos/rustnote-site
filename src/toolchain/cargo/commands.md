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
