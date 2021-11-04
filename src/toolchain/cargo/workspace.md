
# Cargo Workspace

## File Structure

```markdown
Cargo.toml
Cargo.lock
release/
adder
	├── Cargo.toml
	├── src/
add-one
	├── Cargo.toml
	├── src/
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



```rust
println!("Wow cool")
println!("Wow cool")
println!("Wow cool")
```


Must publish each crate individually
