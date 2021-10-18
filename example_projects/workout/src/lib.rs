struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// This is all we need to implement to get access
// to all the iterator methods
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
    println!("what is this: {:?}", sum);
    assert_eq!(sum, vec![9, 36, 81, 144, 225])
}
