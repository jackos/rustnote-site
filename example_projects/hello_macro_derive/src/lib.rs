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
    println!("ast = {:?}", &ast);
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
