use std::thread;
use std::time::Duration;

fn generate_workout(intensity: u32, random_number: u32) {
    println!("Run closure.....");
    let expensive_closure = |num| {
        println!("calculating slowly....");
        thread::sleep(Duration::from_secs(2));
        num
    };
    let x = Cacher::new(expensive_closure);
    let val = x.value.expect("No value exists yet");
    println!("The value is: {}", val);
    if intensity < 25 {
        println!("In if check...");
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity))
        }
    }
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    let intensity = 10;
    let simulated_random_number = 7;
    generate_workout(intensity, simulated_random_number);
}
