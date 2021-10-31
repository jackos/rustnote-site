
# Variables
## Terms
- Place: a location that can hold a value on the stack, heap, registers etc.
- Pointer: An address to a place
- Variable: named value slot on the stack
- Value: combination of type and an element from that types domain of values
- Representation: Used for a type to turn a sequence of bytes to a value

### Example 1
Using that terminology lets break down a simple program

```rust
let x: u32 = 10;
let px: &u32 = &x;
println!("x: {}", px);
```
```output
x: 10
```

Create `variable` named x with `value` 10. The stack now has a `place` with a sequence of bytes that can be converted back into a `value` using the u32 types `representation`

```rust
let x: u32 = 10;
```

Create a `variable` named px that contains a `value` of type `pointer` to the memory address of the `place` where we created the `value` 10

```rust
let px = &x;
```

Dereference the `variable` named px to access the `place`, printing its `value` by using the u32 types `representation` to convert it from a sequence of bytes to a value

```rust
println!("x: {}", px);
```

the `println` macro in rust will dereference any values that are pointers, so you can omit the `*` in `*px`. To print the actual address (the `value` of the `pointer`) try:

```rust
println!("x: {:p}", px);
```

### Example 2
Important to clearly understand what the value is an a `pointer` type:

```rust
let x: u32 = 10;
let y: u32 = 20;
let px1 = &x;
let mut px2 = &x;
println!("px1 value: {:p}", px1);
println!("px1 dereferenced: {}\n", px1);
println!("px2 value: {:p}", px2);
println!("px2 dereferenced: {}\n", px2);
px2 = &y;
println!("px2 value: {:p}", px2);
println!("px2 dereferenced: {}", px2);
```
```output
px1 value: 0x7ffe15aa8178
px1 dereferenced: 10

px2 value: 0x7ffe15aa8178
px2 dereferenced: 10

px2 value: 0x7ffe15aa817c
px2 dereferenced: 20
```
