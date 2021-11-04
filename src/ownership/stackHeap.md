# Stack and Heap
## Terms
`Frame` Allocated at the top of the stack when a function is called, contains all variables within a function and any arguments it takes. Once the function returns, its stack frame is reclaimed. This is directly tied to lifetimes, where you can't reference a value if it's in a frame that has been reclaimed.


## Stack

LIFO

Stores value in the order it gets them, removes values in the opposite order.

Data must have a known fixed size 

Example
```rust
let x = "I'm on the stack!";
```
Immutable fixed size, written directly into the exe

## Heap
Unknown size at compile time, can grow and shrink at runtime.

Memory allocator finds empty spot that's big enough, marks it as being in use, and returns a pointer. This is called allocating.

Values will live until they're explicitly deallocated, which is useful if you need a value to live beyond the lifetime of a functions frame.

The primary mechanism for interacting with the heap is a `Box::new(value)`

If you forget to deallocate it will stick around forever, aka `leaking memory`. Sometimes you want this happen e.g. file configs, in this case `Box::leak(value)` gives back a \``static` reference.



Example
```rust
let mut s = String::from("I'm on the heap!");
```
Mutable can change at runtime so goes onto the heap

## Memory Layout
```rust
let s1 = String::from("hello");
```

### Stack
|   name   |  value  |
|:--------:|:-------:|
| ptr      | address |
| len      | 5       |
| capacity | 5       |

### Heap
| index | value |
|-------|-------|
| 0     | h     |
| 1     | e     |
| 2     | l     |
| 3     | l     |
| 4     | o     |

if we do 
```rust
let s2 = s1;
```
We are copying the stack data, not the heap data. So we get back a pointer to the same heap data.

## Static Memory
Resides in the programs binary code, this contains anything declared with `static` and other constant values such as `&str`. This means the value lives for the duration of the program, getting the lifetime of \``static`, anything pointing to one of these values also gets \``static`. 
