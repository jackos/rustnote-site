
# rand
## Random int (1-100)

```rust
use rand::Rng;
let secret_number = rand::thread_rng().gen_range(1..101);
// `gen_range(1..=100)` is the same as `gen_range(1..101)`
```
```output
Compiling output v0.0.1 (/tmp)
error[E0432]: unresolved import `rand`
 --> main.rs:4:5
  |
4 | use rand::Rng;
  |     ^^^^ use of undeclared crate or module `rand`

error[E0433]: failed to resolve: use of undeclared crate or module `rand`
 --> main.rs:5:21
  |
5 | let secret_number = rand::thread_rng().gen_range(1..101);
  |                     ^^^^ use of undeclared crate or module `rand`

Some errors have detailed explanations: E0432, E0433.
For more information about an error, try `rustc --explain E0432`.
error: could not compile `output` due to 2 previous errors
```



```rust
let x = 15;

println("x = {}", x);
```

