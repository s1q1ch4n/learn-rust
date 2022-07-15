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
`cargo install cargo-expand`  
`rustup toolchian install nightly-x86_64-unknown-linux-gnu`  
`cargo expand`
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