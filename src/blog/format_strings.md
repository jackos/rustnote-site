# Format Strings in Rust 1.58 

### 14th Jan 2022

The `Rust 1.58.0` update today bought a very nice addition, this is a feature that's often missed by the growing number of dynamic language users that are giving Rust a try, and it's now available in stable!

It allows you to put variables from the outside scope directly into format string curly braces:

```rust,noplayground
// In all examples below x = "world"
let x = "world";
println!("Hello {x}!");
```
```output
Hello world!
```
You can also use format specifiers within the curly braces.

For example with debug output:

```rust,noplayground
let items = vec![10, 20, 30];
println!("{items:?}")
```
```output
[10, 20, 30]
```
Or pretty print the output:

```rust,noplayground
println!("{items:#?}")
```
```output
[
    10,
    20,
    30,
]
```
If you haven't seen it before, you can set the minimum width of how items are printed to give uniform spacing with `:[width]`. Example to print a table with even spacing: 

```rust,noplayground
let items = ["these", "words", "are", "different", "sizes"];
let column1 = "item";
let column2 = "iter";
println!("{column1:10}| {column2}");
println!("----------------");
for (i, item) in items.iter().enumerate() {
	println!("{item:10}: {i}");
}
```
```output
item      | iter
----------------
these     : 0
words     : 1
are       : 2
different : 3
sizes     : 4
```
Align items to the centre:

```rust,noplayground
println!("----------------");
for (i, item) in items.iter().enumerate() {
	println!("{item:^10}: {i}");
}
```
```output
----------------
  these   : 0
  words   : 1
   are    : 2
different : 3
  sizes   : 4
```
Align items to right

```rust,noplayground
println!("----------------");
for (i, item) in items.iter().enumerate() {
	println!("{item:>10}: {i}");
}
```
```output
----------------
     these: 0
     words: 1
       are: 2
 different: 3
     sizes: 4
```
Set width 7 characters wide leaving 2 spaces after `world`:

```rust,noplayground
println!("hello {x:7}!");
```
```output
hello world  !
```
Use an existing i32 variable to do the same thing, just put a `$` after the variable name

```rust,noplayground
let spaces = 7;
println!("hello {x:spaces$}!");
```
```output
hello world  !
```
Fill in gaps with any character:

```rust,noplayground
println!("right aligned: hello{x:->7}!");
println!("left aligned: hello{x:-<7}!");
println!("center aligned: hello{x:-^7}!");
```
```output
right aligned: hello--world!
left aligned: helloworld--!
center aligned: hello-world-!
```
Always print the sign of a numeric type even if positive:

```rust,noplayground
let y = 10;
println!("{y:+}");
```
```output
+10
```
Print to hex, binary or octal:

```rust,noplayground
println!("hex: {y:#x}");
println!("binary: {y:#b}");
println!("octal {y:#o}");
```
```output
hex: 0xa
binary: 0b1010
octal: 0o12
```
Set float precision (it rounds to the set precision)

```rust,noplayground
let z = 5.123456;
println!("3 precision: {z:.3}");
println!("5 precision: {z:.5}");
```
```output
3 precision: 5.123
5 precision: 5.12346
```
You can use an existing variable to set the precision:

```rust,noplayground
let precision = 3;
println!("3 precision: {z:.precision$}");
```
```output
3 precision: 5.123
```
Chain different format specifiers together

(you can edit this cell and run it to experiment)
```rust,editable
fn main() {
  let f = 255.555555;
  let dec = 2;
  let width = 10;
  println!("{f} to {dec} decimal places is {f:-^width$.dec$} very cool!");
}
```
Remember that Rust doesn't use any localization, so these outputs will always look the same.

Also to escape these curly braces, just put two of them in front of eachother

```rust,noplayground
println!("Sometimes I need to print {{ or }} too!")
```
```output
Sometimes I need to print { or } too!
```
This quality of life improvement is significant, the first thing a programmer does when learning a new language is print output, this brings Rust on par with the most ergonomic of dynamic languages. Compiled languages can have nice things too!

Thanks for reading, if you have suggestions for things to add you can make pull requests on this file: 

[Github link](https://github.com/jackos/rustnote-site/blob/master/src/blog/format_strings.md)
