Learning Points
===============

Going through the work on:

https://doc.rust-lang.org/book/


## 1_hello_world

URL: https://doc.rust-lang.org/stable/book/ch01-02-hello-world.html

* `fn main() { }` - Main class/start
* `rustc main.rs` - Compiles the .rs file
* `println!();` - Print line, `!` is used for macros

## 2_hello_cargo

URL: https://doc.rust-lang.org/stable/book/ch01-03-hello-cargo.html

* `cargo build` - Will build the executable
* `cargo run` - Will compile and run
* `cargo check` - Will check the source is good
* `cargo build --release` - Will build in release mode

## 3_guessing_game

URL: https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html

* `use std::io;` - Importing the `io` library in `std`
* `let` string creates variables, immutable by default
* `//` is a comment line starter
* `String` is a growable UTF8 encoded bit of text
* `::` indicates a function of the previous string,
so `String::new` calls the new function of `String`
* `&mut guess` Passes the guess variable in as a mutable reference
* `Result` is a possible type returned from functions and is
a type of enum
* Each possible state of an enum is called a variant
* `Result` is used to encode error information
* `Result`'s enum types are `Ok` and `Err`
* `Ok` variant indicates that the result was successful and the value
 is the successful value of calling the method/etc
* `Err` variant indicates that the result failed and contains all
the information about why it failed, etc.
* `expect()` is a function which will. for an `Ok` result will
return the value in `Ok` but otherwise stop the program with the message
passed to the function with the various stack information 


## 4_variables

URL: https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html

* By default variables are immutable
* `mut` can be used to make variables mutable
* Constants can be defined with `const` and can never be mutable
* Constants must have a type annotation and can only be set to values
that can be evaluated at compile time.
* Variables can be shadowed in the same or different scopes with differing
types and other data

## 5_datatypes

URL: https://doc.rust-lang.org/stable/book/ch03-02-data-types.html

* Lots of standard data types
* `_` can be used as a number separator `1_000_000`
* Tuples can be defined with brackets `let tup : (bool, char, i128) = (true, 'W', 1231);`
* They can be unpacked with let and ( ) `let (w, x, y, z) = tup;`
* Arrays are fixed size and can be typehinted `let array : [i64; 3] = [1, 3, 4];`

## 6_functions

URL: https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html

* Function parameters requires the typehinting for each parameter
* Statements are instructions which perform some action and do not return
a value e.g. `let x = 10;`
* Expressions evaluate to a value e.g. `x + 10`
* Expressions do not end in a semicolon
* Calling a function or macro is an expression
* Functions that return values must be typehinted with an arrow `->`
e.g. `fn five() -> i32 { }`




