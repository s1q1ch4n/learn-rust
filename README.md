# learn-rust
* Rust is blazingly fast and memory-efficient: who no runtime or garbage collector.
* Rust's rich type system and ownership model guarantee memory-safety and thread-safety. 
* Rust is a modern systems programming language.
A modern language is a language that solves a lot of the problems in its predecessors.
  * Memory safe
  * No Null
  * No Exceptions
  * Modern package manager
  * No Data Races
## cargo-expand
```
cargo install cargo-expand 
rustup toolchain install nightly-x86_64-unknown-linux-gnu  
cargo expand
```

## Memory Management
* The Stack
* The Heap
* Pointers
* Smart Pointers
### What is the Stack
* It's a special region of the process memory that stores variables created by each function.
* For every function call, a new stack frame is allocated on top of the current one.
* The size of every variable on the stack hash to be known at compile time.
* When a function exits, it's stack frame is released.
### What is the heap
* It's a region of the process memory that is NOT automatically managed.
* It has no size restrictions.
* It's accessible by any function, anywhere in the program.
* Heap allocations are expensive, and we should avoid them when possible.
## Basic Data Types
* Booleans (1 byte)
* Characters (4 byte)
* Integers
* Floats
### Ownership
* Each value in Rust is owned by a variable.
* When the owner goes out of scope, the value will be deallocated.
* There can only be ONE owner at a time.
### Reference
Reference allows us to refer to a value without taking ownership of it.

### &str
`&str` is called a string slice of string. It's an immutable reference to a part of a string.
 A sting slice is a view inside an existing string.  
字符串字面量存在堆上，&str是对字符串一部分的引用。这样做的好处的是，如果经常使用（不修改）该字符串（或其中的一部分），不用再创建（相同的字符串存多份），
节省量空间。
```rust
let string_literal = "1234"; // 包含了整个字符串的字符串切片
```
为什么是字符串切片而不是字符串？  
* 因为字符串可以在运行时动态地增长或缩短，而字符串切片是不可变的。
* 我们不想有按下标访问字符串的行为（如`string[10]`），因为在rust中，所有的字符串都是UTF-8编码，我们不能确定字符串中一个字符总是占一个字节。
正如我们所知道的，所有ASCII字符是占一个字节。但是如果是中文、日文、emoji，它们占多个字节。因此，`string[10]`这种写法并不是总是获取第10个字符，
* 而是获取第10个字节。

## Enum
Enums are special types which have a finite set of values.

## module
* every file in rust is treated as a module.


