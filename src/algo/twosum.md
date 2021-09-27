# Two Sum

```rust
use std::collections::HashMap;
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut h: HashMap<i32, i32> = HashMap::new();
    for (i, v) in nums.iter().enumerate(){
        match h.get(&(target - v)) {
            Some(&i2) => return vec![i2, i as i32],
            None => h.insert(*v, i as i32),
        };
    }
    println!("{:?}", h);
    vec![0, 0]
}

fn main() {
   let result =two_sum(vec![1,5,6,7,8,15], 12);
   println!("{:?}", result);
}
```
