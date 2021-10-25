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