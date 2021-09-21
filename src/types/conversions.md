## Conversions

### Convert string to int

```rust
let mut x = String::new();
stdin().read_line(&mut x).expect("Failed to read line");
let x: u32 = x.trim().parse().expect("Please type a number!");
```

### Convert int to string

```rust
let x: i32 = rand::thread_rng().gen_range(1..101);
let x = x.to_string();
```
