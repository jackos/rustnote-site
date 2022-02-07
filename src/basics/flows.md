
# Flows


`flow` - a variable being assigned a value, to when it's dropped 
```rust
// No flow exists yet
let mut x;
// Flow from x created
x = 42;

// Drawn from x's flow
let y = &x;

// 2nd flow from x created
x = 43;


// Can print `x` because its flow hasn't been conflicted by `y`
println!("{x}");
```
```output
43
```

Trying to print `y` will result in an error, as there are two parallel flows with access to a mutable value
```rust
println!("{y}");
```
```output
Compiling output v0.0.1 (/tmp/rustnote)
warning: value assigned to `x` is never read
  --> src/main.rs:12:1
   |
12 | x = 43;
   | ^
   |
   = note: `#[warn(unused_assignments)]` on by default
   = help: maybe it is overwritten before being read?

error[E0506]: cannot assign to `x` because it is borrowed
  --> src/main.rs:12:1
   |
9  | let y = &x;
   |         -- borrow of `x` occurs here
...
12 | x = 43;
   | ^^^^^^ assignment to borrowed `x` occurs here
...
16 | println!("{y}");
   |           --- borrow later used here

For more information about this error, try `rustc --explain E0506`.
warning: `output` (bin "output") generated 1 warning
error: could not compile `output` due to previous error; 1 warning emitted
```
