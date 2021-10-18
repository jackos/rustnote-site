# Cargo Workspace

## File Structure
```
Cargo.toml
-- add-one
---- src
---- Cargo.toml
-- adder
---- src
---- Cargo.toml
```

## Top level `Cargo.toml`

```toml
[workspace]
members = ["adder", "add-one"]
```

## Depend on local crate in `Cargo.toml`

\#adder/Cargo.toml
```toml
[dependencies]
add-one = { path = "../add-one" }
```

## Special Workspace Commands
### Test single package
```fish
cargo test -p add-one
```

Must publish each crate individually
