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