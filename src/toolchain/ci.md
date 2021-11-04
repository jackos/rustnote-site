# Continuous Integration
## CI Steps for rust
### Tests
```
cargo test
```
### Code Coverage
```
cargo install cargo-tarpaulin
cargo tarpaulin --ignore-tests
```
### Linting
```bash
cargo clippy
```
fail linter check on warnings
```bash
cargo clippy -- -D warnings
```
Ignore clippy on line
```rust
#[allow(clippy::lint_name)]
```
### Formatting
```bash
cargo fmt -- --check
```
### Security Vulnerabilities
```bash
cargo install cargo-audit
cargo audit
```
### Additional Checks
`cargo-deny` identifies unmaintained crates, rejects unwanted licenses, spots reuse of crates with different version

