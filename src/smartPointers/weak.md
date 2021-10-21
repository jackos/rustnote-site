# Weak
To use a `Weak<T>` value you must call `borrow()` to check if it's still in scope. This avoids circular references, below is an example where it's useful when we want two way references so you can get to a parent from the child:
```rust
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    let b = leaf.parent.borrow().upgrade();

    if let Some(node) = b {
        println!("banch value: {}", node.value);
    }

	println!(
        "branch refs - strong: {} weak: {}",
        Rc::strong_count(&branch),
        Rc::weak_count(&branch)
    )
}
```
This gets the value of the branch from the child after checking it's still there by using `upgrade()`

The branch still goes out of scope after the main function ends because its strong reference count remains at 1