# Variables

## Immutable
`const` changes out values at compile time
```rust
const X: i32 = 5;
```

## Mutable
`let` declares the variable, but you won't be able to modify it unless you set `mut`
```rust
let mut x = 5;
x = 6;
```

## Shadow variable
Rust allows you to redeclare a variable by reusing the `let` keyword. This is useful when converting types, so you don't have to use prefixes or suffixes.
```rust
let spaces = "   ";
let spaces = spaces.len();

println!("{}", spaces)
```