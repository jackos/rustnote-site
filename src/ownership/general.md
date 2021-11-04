# General

## Ownership
- Each value has a variable that's called its owner
- There can only be one owner at a time
- When the owner goes out of scope, the value is dropped

## Copying
- If you copy a variable that is an owner e.g. `let x = y`, y becomes invalidated now that x is the owner. In other langauges this might be called a `shallow copy` but in rust it is actually a `move`.

- Types that implement a Copy trait, will retain the older variable when assigned to a new variable. Types with a drop trait can not implement a copy trait.

- Tuples will copy if all items implement a Copy trait, a mix of Drop and Copy will result in just the stack data (pointer, len and capacity) being copied, and the previous variable being invalidated.

## Function move vs copy
Here we see a String type that implements drop, it loses ownership so is invalidated when passed to a function. A type implementing copy doesn't have that problem.
```rs
fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    // s can no longer be used

    let i = 5;
    makes_copy(i);
    // i can still be used
    println!("{}", i);
}

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(i: i32) {
    println!("{}", i);
}
```

## Reassign string to regain ownership
```rust
fn main() {
    let s = String::from("hello");
    let s = takes_ownership(s);
    println!("{}", s);
}

fn takes_ownership(mut s: String) -> String {
    println!("{}", s);
    s.push_str(", world!");
    s
}
```
## Return ownership with tuple
```rust
fn main() {
    let s1 = String::from("hello");
    let (s1, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s1, &len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
```

## Pass a reference to retain ownership
This is called borrowing, can't modify the borrowed reference
```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("length of {}: {}", s1, len)
}

fn calculate_length(s: &String) -> usize {
    s.len()
    // s isn't dropped because it doesn't have ownership
}
```

## Mutate borrowed variable
Needs to be marked as mutable in the: 
- declaration
- argument
- parameter
```rust
fn main() {
    let mut s1 = String::from("hello");
    change_string(&mut s1);
    println!("result is: {}", s1)
}

fn change_string(s: &mut String) {
    s.push_str(", world!");
}
```

## Only one mutable borrow at a time
Won't compile as having two references could cause data races, as soon as a mutable borrow is applied, even immutatble borrows can't be in the same scope.
```rust
let mut s = String::from("hello");
let c1 = &mut s;
let c2 = &s;
c1.push_str(" cool dude");
println!("{}", s)
```
Putting in a different scope makes it OK
```rust
let mut s = String::from("hello");
{
    let c1 = &mut s;
    c1.push_str(" cool dude");
}
// c1 is dropped here so the reference no longer exists
let c2 = &mut s;
c2.push_str(" sweet as cuz");
println!("{}", s)
```
Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used. This will compile no problem:
```rust
let mut s = String::from("hello");

let c1 = &mut s;
c1.push_str(" cool dude");
let c2 = &mut s;
println!("Wow this works fine after mutate: {}", c2);
let c3 = &mut s;
println!("And also after read: {}", c3)
```

## Order of dropping
1) Variables and arguments are dropped in reverse order `why` later value may reference earlier one
2) Nested values are dropped in source code order `why` More intuitive

## Other notes
Mut refs can be used like an owner, except the owner is responsible for dropping value, and so you can't move the value it's referencing unless you leave another value in its place.