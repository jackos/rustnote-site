## String

```rust
let mut s = String::from("hello");
```

`String::from` requests the memory it needs

Deep copy to s2 (copy stack and heap data)

```rust
let s2 = s1.clone();
```

Create an array of bytes from the string

```rust
let bytes = s.as_bytes();
```

## String Slice

String literals are exmaples of string slices `&str`, it is a pointer to a place in memory with a length. The literals point to a place in the binary.

A `String` can be dereference coerced to `&str` as it just takes a slice of the entire String.

A `String` in memory looks like this:

| name     | value   |
| -------- | ------- |
| ptr      | address |
| len      | 11      |
| capacity | 11      |

Whereas a string slice `&str` looks like this:

| Name | value   |
| ---- | ------- |
| ptr  | address |
| len  | 5       |

## General Slice

General slices work the same way, it's just a pointer with a length

```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];

println!("{:?}", slice)
```
## Build a string with format!
```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let word = format!("{}-{}-{}", s1, s2, s3);
```
## Loop over string
```rust
let hello = "नमस्ते";
// Characters
for c in hello.chars() {
	println!("{}", c);
}
// Bytes
for b in hello.bytes() {
	println!("{}", b);
}
```

## Multiline literals
Escape first newline with `\`
```rust
let contents = "\
Rust:
safe, fast, productive.
Pick three.";
println!("{}", contents)
```