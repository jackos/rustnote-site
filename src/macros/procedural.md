# Procedural Macros
Generates code from attributes.

Code must live in its own special crate type.

## Create a procedural macro
```rust
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
}
```
## Example
### hello_macro_derive/Cargo.toml
```toml
[package]
name = "hello_macro_derive"
version = "0.1.0"
edition = "2021"

[lib]
proc-macro = true

[dependencies]
syn = { version = "1.0", features = ["extra-traits"] }
quote = "1.0"
```
### hello_macro_derive/lib.rs
```rust
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
    // #name replaces value in println! below, it works with repetition
    impl HelloMacro for #name {
            fn hello_macro() {
                // Stringify turns an expression into a string literal
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
```














### pancakes/Cargo.toml














```toml
[package]
name = "pancakes"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
hello_macro_derive = { path = "../hello_macro_derive" }
```














### pancakes/main.rs














```rust
use hello_macro_derive::HelloMacro;

trait HelloMacro {
    fn hello_macro() {}
}

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}
```
```output
Compiling output v0.0.1 (/tmp)
error[E0432]: unresolved import `hello_macro_derive`
 --> main.rs:4:5
  |
4 | use hello_macro_derive::HelloMacro;
  |     ^^^^^^^^^^^^^^^^^^ use of undeclared crate or module `hello_macro_derive`

error: cannot determine resolution for the derive macro `HelloMacro`
  --> main.rs:10:10
   |
10 | #[derive(HelloMacro)]
   |          ^^^^^^^^^^
   |
   = note: import resolution is stuck, try simplifying macro imports

error[E0599]: no function or associated item named `hello_macro` found for struct `Pancakes` in the current scope
  --> main.rs:14:15
   |
11 | struct Pancakes;
   | ---------------- function or associated item `hello_macro` not found for this
...
14 |     Pancakes::hello_macro();
   |               ^^^^^^^^^^^ function or associated item not found in `Pancakes`
   |
   = help: items from traits can only be used if the trait is implemented and in scope
note: `HelloMacro` defines an item `hello_macro`, perhaps you need to implement it
  --> main.rs:6:1
   |
6  | trait HelloMacro {
   | ^^^^^^^^^^^^^^^^

Some errors have detailed explanations: E0432, E0599.
For more information about an error, try `rustc --explain E0432`.
error: could not compile `output` due to 3 previous errors
```


```rust
println!("{}", x);
```
```output
Compiling output v0.0.1 (/tmp)
error[E0425]: cannot find value `x` in this scope
 --> main.rs:4:16
  |
4 | println!("{}", x);
  |                ^ not found in this scope

For more information about this error, try `rustc --explain E0425`.
error: could not compile `output` due to previous error
```


```rust
use regex::Regex;
let x = 10;
println!("Wow nice");

let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();

```
```output
Wow nice
```


```rust
use smallvec::{SmallVec, smallvec};
    
// This SmallVec can hold up to 4 items on the stack:
let mut v: SmallVec<[i32; 4]> = smallvec![1, 2, 3, 4];

// It will automatically move its contents to the heap if
// contains more than four items:
v.push(5);

// SmallVec points to a slice, so you can use normal slice
// indexing and other methods to access its contents:
v[0] = v[1] + v[2];
v.sort();

println!("{:?}", v);
```
```output
[2, 3, 4, 5, 5]
```




```rust
use getrandom::getrandom;
fn get_random_buf() -> Result<[u8; 32], getrandom::Error> {
    let mut buf = [0u8; 32];
    getrandom::getrandom(&mut buf)?;
    Ok(buf)
}for x in vec![1, 2, 4, 5] {
	println!("Wow: {}", x);
}
```

