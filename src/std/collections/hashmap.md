# HashMap
```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert(1, 2);
map.insert(2, 3);
```

### Loop
Iteration is in an arbitrary order
```rust
for (key, value) in &map {
    println!("{}: {}", key, value);
}
```

## Insertion types
### Insert
Overwrites if exists
### Entry (do something if doesn't exists)
```rust
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
// Doesn't do anything
scores.entry(String::from("Blue")).or_insert(20);
// Sets Yellow to 20
scores.entry(String::from("Yellow")).or_insert(20);
println!("{:?}", scores);
```
### Update based on old value
Example of increasing word count based on how many times it's been seen
```rust
use std::collections::HashMap;
fn main() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map)
}
```