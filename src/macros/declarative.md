# Declarative Macros
## Aka
- `Macros by example`
- `macro_rules!`
- `macros`

## Simplified `vec` example
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
