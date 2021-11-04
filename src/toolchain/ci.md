# Continuous Integration
## CI Steps for rust
### Tests
```
cargo test
```
### Code Coverage
`cargo install cargo-tarpaulin`
`cargo tarpaulin --ignore-tests`

### Linting
- `cargo clippy` 
- `cargo clippy -- -D warnings` fail linter check on warnings
- `#[allow(clippy::lint_name)]` Ignore clippy on line

### Formatting
`cargo fmt -- --check`

### Security Vulnerabilities
`cargo audit`

### Additional Checks
`cargo deny` identifies unmaintained crates, rejects unwanted licenses, spots reuse of crates with different version

