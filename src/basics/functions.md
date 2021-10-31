# Functions

## Statement vs Expression
```plaintext
statement - instruction that performs an action
expression - evaluate to resulting value
```
## Function
```rust
fn add(a: i32, b: i32) -> i32{
    // No semicolon means it returns this value
    a + b 
}
let x = add(2, 5);
println!("x: {}", x);
``````output
x: 7
```

x: 7
```markdown

x: 7
```

## Block evaluation
Rust will consider opening and closing brackets a new scope, and allow you to return values from within.
```markdown
fn main() {
    let y = {
        let x = 10;
        x + 1
    };
    println!("{}", y)
}
```
error: could not find `Cargo.toml` in `/home/jacko/vimwiki/rust/src/basics` or any parent directory
```markdown

`x + 1` has no semicolon, and so it returns the value. If it had a semicolon it would be a statement and give you an error that it's returning `()` which can be thought of as an empty tuple denoting that nothing is returned.

Rust returns the last expression in a block or function implicitly.
```