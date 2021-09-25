# Attributes

## derive
Add Debug to allow the program to print the structure of custom types, enums and structs with {:?} 
```rust
#[derive(Debug)]
```
## Derivable traits
### Debug
Required for `assert_eq` and sets what is printed when using `println!("{:?}", customType`

### PartialEq
Determines what causes two types to be `==` or `-=` to each other, by setting the functions `eq` and `ne`

### PartialOrd
Determines the order when comparing complex data types with `>` `>=` `<` `<=`.The order of structs and enums from top to bottom matches smallest to largest when deriving `PartialOrd`

### Eq
Determines when 

## cfg
Mark a function as a test so it doesn't produce warnings about unused `use` directives. 

TODO: Probably does more not sure yet.
#[cfg(test)]
```rust
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
```

#### PartialEq Example
This uses the derivable trait to set the default equality check `PartialEq` on the enum `Size` and then overwrites it on the struct `Dog` so only hair has to match for a Dog to be equal to another one.
```rust
#[derive(Debug, PartialEq)]
enum Size {
    Big,
    Small,
}

#[derive(Debug)]
struct Dog {
    hair: String,
    size: Size,
}

impl PartialEq for Dog {
    fn eq(&self, other: &Self) -> bool {
        self.hair == other.hair
    }
}

fn check_equal(a: &Dog, b: &Dog) -> bool {
    if a == b {
        return true;
    }
    false
}

fn main() {
    let ben = Dog {
        hair: String::from("Golden"),
        size: Size::Big,
    };

    let axel = Dog {
        hair: String::from("Golden"),
        size: Size::Small,
    };

    let eq = check_equal(&ben, &axel);
    println!("Are {:?} and {:?} equal: {}", ben, axel, eq);
}
```

#### PartialOrd example
```rust
#[derive(Debug, PartialEq, PartialOrd)]
enum Size {
    Small,
    Medium,
    Large,
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Doggy {
    size: Size,
    breed: String,
}

fn main() {
    let lab = Doggy {
        breed: String::from("Lab"),
        size: Size::Large,
    };
    let pug = Doggy {
        breed: String::from("Pug"),
        size: Size::Small,
    };

    println!(
        "Is a {} bigger than a {}: {}",
        lab.breed,
        pug.breed,
        lab > pug
    );
}
```