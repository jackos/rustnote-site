## Tuples

### Print tuple items

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
println!("{:?}", tup);
// Pretty print
println!("{:#?}", tup);
```

### Destructure tuple

```rust
let tup = (500, 6.4, 1);
let (x, y, z) = tup;
println!("The value of y is: {}", y);
```

### Access item in tuple

```rust
let x = (10, 10.5);
let y = x.1;
println!("{}", y);
```
