# Attribute Macros
These are the same as `Procedural Macros` except:

- Allow you to create new attributes
- Can be applied to more than just structs and enums
- Don't use `derive`

## Example
```rust
#[route(GET, "/")]
fn index(){}
```

This would be defined like
```rust
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
```
`attr` Takes in the `GET, "/"` part, and `item` takes in the body, which is the function in this case.



