# Macros
## Declarative Macros
	- `Macros by example`
	- `macro_rules!`
	- `macros`

### Simplified `vec`
```rust
// indicates macro is in scope if crate is in scope
#[macro_export]
macro_rules! vec2 {
    // $()     = pattern to match
    // $x:expr = match any expression and name it $x
    // ,*      = one or more matches
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    let v: Vec<u32> = vec2![1, 2, 3];
    println!("v = {:?}", v);
}
```
the block inside `{$()*}` results in the macro expanding, so it would become:
```rust
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
```

## Procedural Macros
Generates code from attributes.

Code must live in its own special crate type.

### Create a procedural macro
```rust
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
}
```

### Example
```rust
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}
```

## Full custom derive macro example