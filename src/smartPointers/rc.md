# Rc<T> Reference Counted Smart Pointer
Used for multiple ownership e.g. if multiple edges are pointing to a value in a graph.

## Cons list with reference counting
Here `a` is referenced by `b` and `c`
```rust
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    println!("{:?}\n{:?}\n{:?}", a, b, c);
}
```
`Rc::clone(&a)` doesn't create a deep copy, it only increments the reference count. It's the same as calling `a.clone()`, but most implementations create a deep copy, so by convention we distinguish it's coming from `Rc`.

`Rc::strong_count` shows the amount of references.

