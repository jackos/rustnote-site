# RefCell
Allows for interior mutability without making the value mutable to outside code.

Example of using a mock object for testing purposes, where we need to mutate the test object, but we still want to implementation to remain immutable.
```rust
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: you are over your quota!");
        }
        if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!")
        }
        if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota")
        }
    }
}

fn main() {
    println!("Wow")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(100);
        println!("Print work: {:?}", mock_messenger.sent_messages);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 3);
    }
}
```
A RefCell has the same rules as normal borrowing, where it can only have one mutable borrow or many immutable borrows. But if the rules are violated, the code will still compile but panic at runtime:
```rust
impl Messenger for MockMessenger {
	fn send(&self, message: &str) {
		let mut one_borrow = self.sent_messages.borrow_mut();
		let mut two_borrow = self.sent_messages.borrow_mut();

		one_borrow.push(String::from(message));
		two_borrow.push(String::from(message));
	}
}
```
This throws a panic: `'already borrowed: BorrowMutError'`


## Combining `Rc` and `RefCell`
This example shows how to keep multiple references to a value that we can still mutate
```rust
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::{cell::RefCell, rc::Rc};

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
```