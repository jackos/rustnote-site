# Variables

## Immutable
```rust
const X: i32 = 5;
let x = 5;
// Can't change values, const values are changed at compile time
```

## Mutable
```rust
let mut x = 5;
x = 6;
```

## Shadow variable
```rust
let spaces = "   ";
let spaces = spaces.len();

println!("{}", spaces)
```