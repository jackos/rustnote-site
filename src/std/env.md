# std env
## Get environment variable
```rust
let editor = env::var("EDITOR").expect("EDITOR doesn't exist");
println!("{}", editor);
```