


# Function-like Macros
Same as declarative macros, but they take a `TokenStream` like a procedural macro does, so you can use an `ast`

## Example



```rust
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
```




Which can then be used like



```rust
let sql = sql!(SELECT * FROM posts WHERE id=1);
```
```output
Compiling output v0.0.1 (/tmp)
error: cannot find macro `sql` in this scope
 --> main.rs:4:11
  |
4 | let sql = sql!(SELECT * FROM posts WHERE id=1);
  |           ^^^

error: could not compile `output` due to previous error
```


```rust
let x = 5;
```



