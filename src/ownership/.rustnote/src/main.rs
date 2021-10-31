#![allow(dead_code)]
fn main() {
let x: u32 = 10;
let px: &u32 = &x;
let x: u32 = 10;
let px = &x;
println!("x: {:p}", px);
}