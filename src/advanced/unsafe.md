# Unsafe
## Dereferencing a Raw Pointer
Immutable raw pointer: `*const T` mutable raw pointer: `*mut T`

### Reasons why raw pointers are unsafe
- Ignore borrowing rules e.g. multiple mutable pointers to the same location
- Aren’t guaranteed to point to valid memory
- Are allowed to be null
- Don’t implement any automatic cleanup

### Example
```rust
let mut num = 5;

let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;

unsafe {
	println!("r1: {}", *r1);
	*r2 = 50;
	println!("r2: {}", *r2);
}
```

## Calling an Unsafe Function or Method
```rust
unsafe fn dangerous() {}

unsafe {
	dangerous();
}
```

## Creating a safe abstraction on unsafe code
```rust
use core::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = split_at_mut(&mut v, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}
```

## Calling functions from other languages
Makes `abs` from `C` available
```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

> ## Calling Rust function from another language
> ```
> #[no_mangle]
> pub extern "C" fn call_from_c() {
>     println!("Just called a Rust function from C!");
> }
>
> Once compiled to library can be linked to from C
> ```

## Static Variables (globals)
You can modify global state in unsafe Rust
```rust
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
```

## Unsafe Traits
Must use unsafe keyword to implement an unsafe trait
```rust
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

fn main() {}
```

## Unions
To access fields in a union is unsafe, as there is no guarantee what type it will be. Unions are generally only used for interacting with C code, they're like enums without the safety guarantees.
```rust
#[repr(C)]
union MyUnion {
    f1: u32,
    f2: f32,
}

fn main() {
    let mut u = MyUnion { f2: 1.5 };
    unsafe {
        let f = u.f2;
        println!("proper: {} wrong: {}", u.f2, u.f1);
        u.f1 = 10;
        println!("proper: {} wrong: {}", u.f1, u.f2);
    }
}
