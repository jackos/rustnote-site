use hello_macro_derive::HelloMacro;

trait HelloMacro {
    fn hello_macro() {}
}

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}
