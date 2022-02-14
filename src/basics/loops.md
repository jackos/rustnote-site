
# Loops

## While
```rust
let mut number = 3;
while number != 0 {
	println!("{}!", &number);
	number -= 1;
}
```
```output
3!
2!
1!
```

## While let
Same as while but we get a variable back if the assertion matches, so we can access `top` directly below:
```rust
fn main() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}
```
```output
3
2
1
```

## For destructure
Also using an enumerate to get back the index
```rust
let v = vec!['a', 'b', 'c'];

for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
}
```
```output
a is at index 0
b is at index 1
c is at index 2
```

## For Range
```rust
for number in (1..4).rev() {
    println!("{}!", number);
}
println!("LIFTOFF!!!");
```
```output
3!
2!
1!
LIFTOFF!!!
```

## For iter
Print all the items in the array
```rust
let a = [10, 20, 30, 40, 50];
for e in a.iter() {
		println!("The value is: {}", e);
}
```
```output
The value is: 10
The value is: 20
The value is: 30
The value is: 40
The value is: 50
```

## For .len()
Print just the iterator
```rust
let x = [1, 5, 10, 20];

for i in 0..x.len() {
    println!("{}", i);
}
```
```output
0
1
2
3
```

## For iter enumerate over string as bytes
This example searches for a `b' '` denoting a space to return to first word
```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
		let mut s = String::from("Coolioio one yo");
        // deref coercion
		let x = first_word(&mut s);
		println!("{}", x);
}
```
```output
Coolioio
```

## Break
Break can be suffixed with an expression to return the result of the expression.

```rust
let mut counter = 0;
let x = loop {
		counter += 1;
		if counter == 10 {
				break counter * 2;
		}
};
println!("{}", x);
```
```output
20
```

## Break return
If the semicolon is removed from the end of the loop, we can return the result from the loop expression.
```rust
fn looper() -> i32 {
    let mut counter = 0;
    loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    }
}

fn main() {
    let x = looper();
    println!("The result is {}!", x);
}
```
```output
The result is 20!
```
