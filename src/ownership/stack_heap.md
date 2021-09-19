# Stack and Heap

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
Unkown size at compile time, can grow and shrink at runtime.

Memory allocator finds empty spot that's big enough, marks it as being in use, and returns a pointer. This is called allocating.

Example
```rust
let mut s = String::from("I'm on the heap!");
```
Mutable can change at runtime so goes onto the heap

## Memory Layout
```rust
let s1 = String::from("hello");
```

### Heap
|   name   |  value  |
|:--------:|:-------:|
| ptr      | address |
| len      | 5       |
| capacity | 5       |

### Stack
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