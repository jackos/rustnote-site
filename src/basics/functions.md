# Functions

## Definitions
```text
statement - instruction that performs an action
expression - evaluate to resulting value
```

Block evaluation
```rust
let y = {
        let x = 10;
        x + 1
    };
```
`x + 1` has no semicolon, and so it returns the value. If it had a semicolon it would be a statement.

Rust returns the last expression implicitly
