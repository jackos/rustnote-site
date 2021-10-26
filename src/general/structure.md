
# Project Structure


```rust
let x = vec![1,4,5];
for item in x {
	println!("item: {}", item );
}
```
```output
item: 1
item: 4
item: 5
```


## General Notes
- `src/main.rs` can't be tested so it should be minimal, move any complicated logic into `src/lib.rs`

### Primitive Obsession 
Anti-pattern e.g. returning a tuple and immediately destructuring it is a sign the code can be refactored into a struct e.g.


```rust
fn parse_config(args: &[String]) -> (&str, &str) {
    (&args[1], &args[2])
}
```

Would be better written as

```rust
struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    Config{query: args[1].clone(), filename: args[2].clone()}
}
```


### Struct::new()
It's idiomatic to create a struct using a new() method