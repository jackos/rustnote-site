# rand
## Random int (1-100)
```rust
use rand::Rng;
let secret_number = rand::thread_rng().gen_range(1..101);
// `gen_range(1..=100)` is the same as `gen_range(1..101)`
```
