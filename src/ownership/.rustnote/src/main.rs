#![allow(dead_code)]
fn main() {
let mut x = 42;
// The flow is continued from x
let y = &x;
// Now there are two mutable flows. No error yet
x = 43;
// Th
let mut x = 5;
// y is never used again
let y = &x;
x = 10;
println!("2nd flow x: {}", x)
}