# Advanced Types
## Newtype Definition
Simply wrapping a type in another to hide implementation details. e.g. you could wrap a `HashMap<i32, String>` in  `People` and caller using the public API wouldn't need to know anything about the i32 ID.

## Type Synonyms with Type Aliases
```rust
type Kilometers = u32;
let x: Kilometers = 6;
```
### With a generic type
```rust
type Result<T> = std::result::Result<T, std::io::Error>;
```
Which would simplify the `Write` trait:
```rust
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}
```
## Never Type
Denoted by !, it means the program never returns e.g. an endless loop or match branches that end with `continue` || `panic` 

## DST - Dynamically Sized Type
When using a generic function e.g.
```rust
fn generic<T>(t: T) {

}
```
Is actually treated like this:
```rust
fn generic<T: Sized>(t: T) {

}
```
Meaning it implements the `Sized` trait, so it must have a known size at compile to.

To relax this restriction:
```rust
fn generic<T: ?Sized>(t: &T) {

}