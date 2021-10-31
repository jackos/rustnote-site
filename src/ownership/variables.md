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
```output
x: 10
```
the `println` macro in rust will dereference any values that are pointers, so you can omit the `*` in `*px`. To print the actual address (the `value` of the `pointer`) try:
```rust
println!("x: {:p}", px);
```
```output
x: 0x7ffdc852a9ac
```
### Example 2
Important to clearly understand what the value is in a `pointer` type:
```rust
let x: u32 = 10;
let y: u32 = 20;
let px1 = &x;
// Set the `value` of px2 to an address pointing to the `place` in memory containing the value 10
let mut px2 = &x;
println!("px1 value: {:p} dereferenced: {}", px1, px1);
println!("px2 value: {:p} dereferenced: {}", px2, px2);
// Update the `value` of px2 to an address pointing to the `place` in memory containing the value 20
px2 = &y;
println!("px2 value: {:p} dereferenced: {}", px2, px2);
```
```output
px1 value: 0x7ffd4e2ef4a8 dereferenced: 10
px2 value: 0x7ffd4e2ef4a8 dereferenced: 10
px2 value: 0x7ffd4e2ef4ac dereferenced: 20
```
The part where people get confused is that rust automatically dereferences a `value` on a lot of occasions, go over the previous code to convince yourself that px2's `value` is an `address` of type `&32`, not a `u32`.

## Flows
Sometimes described as `dependency lines`, they track the lifetimes of values.
Looking at a simple program:
```rust
let mut x = 42;
let y = &x;
x = 43;
println!("{} {}", x, y);
```
```output
Compiling output v0.0.1 (/tmp)
error[E0506]: cannot assign to `x` because it is borrowed
 --> main.rs:6:1
  |
5 | let y = &x;
  |         -- borrow of `x` occurs here
6 | x = 43;
  | ^^^^^^ assignment to borrowed `x` occurs here
7 | println!("{} {}", x, y);
  |                      - borrow later used here

For more information about this error, try `rustc --explain E0506`.
error: could not compile `output` due to previous error
```
This fails to compile because there are conflicting `flows`.

When `y` is assigned a reference to `x` as its value a new `flow` is created, there are now two parallel flows
```rust
fn wow_it_works(x: &str) -> String {
	println!("Data coming in: {}", x);
	format!("Very cool: {}", x)
}
```
```rust
let x = wow_it_works("nice one");
println!("{}", x);
```
```output
Very cool: nice one
```