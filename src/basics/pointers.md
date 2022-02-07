
# Pointers
## Terms
- `value`:    combination of a type and relevant element
- `variable`: named value slot on the stack 
- `place`:    can hold a `value` somewhere in memory, either on the stack, heap, registers or disk
- `pointer`:  a value holding an address to a `place`

## Example
This example is to show that a pointer has a value with an address that it's pointing to, but it also has it's own address.

Create two pointers to `i32` and print the pointers, Rust will automatically dereference them:
```rust
let x = 10;
let y = 20;

let mut x_pointer = &x;
let y_pointer = &y;

println!("\nx_pointer dereferenced: {x_pointer}\ny_pointer dereferenced: {y_pointer}");
```
```output
x_pointer dereferenced: 10
y_pointer dereferenced: 20
```

Print out the address that's inside the pointer using the `:p` format specifier:
```rust
println!("\nx address: {x_pointer:p}\ny address: {y_pointer:p}");
```
```output
x address: 0x7ffd532910e0
y address: 0x7ffd532910e4
```

Print the address of the pointer value itself by creating a pointer to the pointer with `&` and printing it with `:p`:
```rust
println!("\nx_pointer address: {:p}\ny_pointer address: {:p}", &x_pointer, &y_pointer);
```
```output
x_pointer address: 0x7fffba86ec48
y_pointer address: 0x7fffba86ec50
```

Change the address that `x_pointer` is pointing to and dereference it
```rust
x_pointer = y_pointer;
println!("\nx_pointer dereferenced: {}", x_pointer);
```
```output
x_pointer dereferenced: 20
```

`x_pointer` and `y_pointer` now point to the same address known as a `shared_reference`
