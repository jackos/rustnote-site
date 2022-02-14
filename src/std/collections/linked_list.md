```rust
use std::collections::LinkedList;
fn main() {
    let mut list = LinkedList::from([1, 2, 3]);
    loop {
        let x = list.pop_front();
        if let Some(i) = x {
            println!("{i}");
        } else {
            break;
        }
    }
}

```
```output
1
2
3
```
