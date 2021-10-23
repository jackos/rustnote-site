# Terms

### Dynamic Dispatch
`Static Dispatch` is when the compiler knows ahead of time what method it'll call.

An example of `Dynamic Dispatch` is trait objects e.g. `Box<dyn Error>`

### Polymorphism
This is not inheritance, it just means code that can work with multiple data types. Rust uses generics and traits for this, which is sometimes referred to as bounded parametric polymorphism.

### Test double
Type used that is used in place of another type during testing

### Mock Object
Types of test doubles that record what happens during a test 

### Green threads
A thread that is different to operating system threads, where there may be more green threads.

### Runtime
The code that a language ships in every binary, when people say `no runtime` they really mean `small runtime` unless it's assembly language

### Duck typing
If it walks like a duck and quacks like a duck, then it must be a duck. Means that we don't need to know what the concrete type is at runtime, we just run a certain method on each type.

## Refutable
This means the pattern may not match, it's used only in `if let` and `while let`. Irrefutable means the pattern will match or the program will fail to compile.