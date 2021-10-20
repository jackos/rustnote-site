# Custom Smart Pointer
Data structure similar to `Box`
```rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
```
Implementing `Deref` trait
```rust
use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```
Now running the dereference operator will return what it's pointing to
```rust
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```
Behind the scenes this is running: `*(y.deref())`

<<<<<<< HEAD
git commit --amend --author "Jack Clayton <jackos@me.com>" --no-edit && \
git rebase --continue
=======
>>>>>>> da38eca (Deref and drop)
