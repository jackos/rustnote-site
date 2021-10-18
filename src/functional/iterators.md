# Iterators

### Basic iterator
```rust
let v1 = vec![1, 2, 3];
let v1_iter = v1.iter();

// This for loop makes v1_iter mutable
// As each iteration changes its internal state
for val in v1_iter {
	println!("Got: {}", val + 1)
}
```

### What `for` does without syntactic sugar
```rust
let x = v1_iter.next();
println!("Got: {}", x.unwrap());
let x = v1_iter.next();
println!("Got: {}", x.unwrap());
let x = v1_iter.next();
println!("Got: {}", x.unwrap());


let x = v1_iter.next();
println!("Loop ended as x == {:?}", x.unwrap());
```
The `x` variables are immutable references, use `into_iter` for mutable references

### Consuming Adaptors
Uses up an iterator by calling `next()`, an example is `sum()`:

```rust
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();

let total: i32 = v1_iter.sum();

println!("total: {}", 6);
```

### Iterator Adaptors
Adds some functionality to each iteration
```rust
let v1 = vec![1, 2, 3];
let v1_iter = v1.iter().map(|x| x * 2);

for x in v1_iter {
	println!("x: {}", x);
}
```

### Iterator Adaptor Examples
#### Map
```rust
.map(|mut x| {
	x.style += "wow";
	x.size *= 10;
	x
}
```

#### Filter
```rust
.filter(|s| s.size == shoe_size)
```

### Roll your own iterator
```rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// This is all we need to implement to get access
// to all `Iterator` methods
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 15 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}


// Example using a few `Iterator` methods
#[test]
fn using_other_iterator_trait_methods() {
    let sum: Vec<u32> = Counter::new()
        // Create pairs from iterator e.g. (1, 1), (2, 2)
        .zip(Counter::new())
        // Multiply the pairs together, which is squaring them
        .map(|(a, b)| a * b)
        // Filter out anything that isn't divisible by 3
        .filter(|x| x % 3 == 0)
        .collect();
    assert_eq!(sum, vec![9, 36, 81, 144, 225])
}
```