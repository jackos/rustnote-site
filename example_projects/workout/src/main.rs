use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;
struct Cacher<T, U>
where
    T: Fn(U) -> U,
{
    calculation: T,
    values: HashMap<U, U>,
}

impl<T, U: Eq + Hash + Copy> Cacher<T, U>
where
    T: Fn(U) -> U,
{
    fn new(calculation: T) -> Cacher<T, U> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }
    fn values(&mut self, arg: U) -> U {
        if !self.values.contains_key(&arg) {
            self.values.insert(arg, (self.calculation)(arg));
        }
        self.values[&arg]
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let closure = |num| {
        println!("calculating slowly....");
        thread::sleep(Duration::from_secs(2));
        num
    };
    let mut expensive_result = Cacher::new(closure);
    // Example only running expensive result closure when needed
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.values("cool"));
        println!("Next, do {} situps!", expensive_result.values("cool"));
        println!("Next, do {} pullups!", expensive_result.values("very cool"));
    } else {
        // Example not having to run expensive calculation at all
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.values("so damn cool")
            )
        }
    }
}
fn main() {
    let intensity = 10;
    let simulated_random_number = 7;
    generate_workout(intensity, simulated_random_number);
}
