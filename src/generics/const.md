```rust
struct ArrayPair<T, const N: usize> {
	left: [T; N],
	right: [T; N],
}

impl<T: Debug, const N: usize> Debug for ArrayPair<T, N> {

}

let x = ArrayPair{
	left: [i32; 12, 5],
	right: [i32; 5, 10],
}
```
```output
Compiling output v0.0.1 (/tmp/rustnote)
error: expected one of `.`, `?`, `]`, or an operator, found `,`
  --> src/main.rs:13:15
   |
12 | let x = ArrayPair{
   |         --------- while parsing this struct
13 | left: [i32; 12, 5],
   |               ^ expected one of `.`, `?`, `]`, or an operator

error: expected identifier, found `5`
  --> src/main.rs:13:17
   |
12 | let x = ArrayPair{
   |         --------- while parsing this struct
13 | left: [i32; 12, 5],
   |                 ^ expected identifier

error: expected one of `.`, `?`, `]`, or an operator, found `,`
  --> src/main.rs:14:15
   |
12 | let x = ArrayPair{
   |         --------- while parsing this struct
13 | left: [i32; 12, 5],
14 | right: [i32; 5, 10],
   |               ^ expected one of `.`, `?`, `]`, or an operator

error: expected identifier, found `10`
  --> src/main.rs:14:17
   |
12 | let x = ArrayPair{
   |         --------- while parsing this struct
13 | left: [i32; 12, 5],
14 | right: [i32; 5, 10],
   |                 ^^ expected identifier

error: expected `;`, found `}`
  --> src/main.rs:15:2
   |
15 | }
   |  ^ help: add `;` here
16 | }
   | - unexpected token

error[E0404]: expected trait, found derive macro `Debug`
 --> src/main.rs:8:32
  |
8 | impl<T: Debug, const N: usize> Debug for ArrayPair<T, N> {
  |                                ^^^^^ not a trait
  |
help: consider importing one of these items instead
  |
2 | use core::fmt::Debug;
  |
2 | use std::fmt::Debug;
  |

error[E0404]: expected trait, found derive macro `Debug`
 --> src/main.rs:8:9
  |
8 | impl<T: Debug, const N: usize> Debug for ArrayPair<T, N> {
  |         ^^^^^ not a trait
  |
help: consider importing one of these items instead
  |
2 | use core::fmt::Debug;
  |
2 | use std::fmt::Debug;
  |

For more information about this error, try `rustc --explain E0404`.
error: could not compile `output` due to 7 previous errors
```
