# Pointer Traits
## Deref Coercion
`String` implements its `Deref` trait by returning an `&str`, so it can be deref coerced:
```rust
fn hello(name: &str) {
    for x in name.chars() {
        println!("Hello: {}!", x);
    }
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
```
This is what `String` Deref looks like:
```
#[stable(feature = "rust1", since = "1.0.0")]
impl ops::Deref for String {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        unsafe { str::from_utf8_unchecked(&self.vec) }
    }
}
```
The `type Target = str;` is what allows the type coercion

Without deref coercion this would have to be written as:
```rust
hello(&(*m)[..]);
```
There is no runtime penalty for this, it is all taken care of at compile time.

### Deref Coercion Cases
1. From &T to &U when T: Deref<Target=U>
2. From &mut T to &mut U when T: DerefMut<Target=U>
3. From &mut T to &U when T: Deref<Target=U>

This means that any type can be coerced to the target, except an immutable can not be coerced to a mutable (this would break borrowing rules)

## Drop
Printing once all references go out of scope
```rust
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
```
Trying to call this directly would result in an `double free` error. Instead if you need to call it early use `drop()`, which is automatically in scope from `std::mem:drop`


