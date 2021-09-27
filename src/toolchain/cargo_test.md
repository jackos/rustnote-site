# Cargo Test
## Help
```bash
cargo test -- --help
```
Gives extra info about options and attributes
## Options
```sh
cargo test -- --test-threads=1
```
Sets test threads to `1` so they don't interfere with each other

```sh
cargo test -- --show-output
```
Allows successful tests to print output

```sh
cargo test foo
```
Filters on `foo`, any test that contains that pattern will run

```sh
cargo test -- --ignored
```
Any tests annotated with `#[ignore]` will run
