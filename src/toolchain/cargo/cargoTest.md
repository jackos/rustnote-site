# Cargo Test
## Help
Extra info about test options and attributes
```bash
cargo test -- --help
```
## Options
Sets test threads to `1` so they don't interfere with each other
```sh
cargo test -- --test-threads=1
```
Allows successful tests to print output
```sh
cargo test -- --show-output
```
Filters on `foo`, any test that contains that pattern will run
```sh
cargo test foo
```
Any tests annotated with `#[ignore]` will run
```sh
cargo test -- --ignored
```
