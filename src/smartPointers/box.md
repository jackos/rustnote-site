# Smart Pointer - Box\<T\>
Only provides indirection from the stack to the heap, no other special capabilities. No performance overhead.

It implements the traits `Deref` and `Drop` which deallocates memory when all references are out of scope.

## Use Cases
- Type without known size at compile time
- Large data transfer ownership without copying
- Own a value that implements a specific trait

### Allocate single i32 to heap
You wouldn't do this in practice, as it's best for small values to be on the stack
```rust
let b = Box::new(5);
println!("b = {}", b)
```

### Construct Function - `cons`
Recursive data construct using pairs, where one item is a single value, and the other is the next item which also contains a pair, until the `Nil` value is hit.

It's not often a useful concept in Rust, but it's good example for a recursive type to show where a `Box` is useful

```rust
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};
fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("{:?}", list)
}
```
