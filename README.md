# Rust (basic)

Going through Rust language and writing modular utils.

## Some Rust Notes

Rust is a statically typed language meaning it must know the types of all variables at compile time, but the cmpiler can usually infer types we want to use based on the value and context of its use (e.g. doing `let x = 1;` , the compiler infer an `i32` Integer type by default).

- Convention for variable names is <u>snake-case</u> (e.g. `snake_case`)
- Variables hold primitive data or references to data
- Variables are immutable by default - you can init a var to be mutable using `mut` ->

```rust
let mut changing_thing = "hello world";
```

- While variables are immutable by default there is a `const` keyword. It requires that you specify the type when initialising it
- You have to use single-quotes (`' '`) to assign a `char` type variable

```rust
let a = 'a';
```

- There are 2 types of strings:
  - <b>Primitive</b> -> Immutable fixed-length strings stored in memory
  - <b>String</b> -> Growable, <u>heap-allocated</u> data structure. Used when you need to modify or own string data
- Tuples group together values of different types <u>(Max 12 elements)</u>
- Arrays are <u>fixed</u>. Fixed in length and types.
- Arrays are stack-allocated

## Rust Primitive Types

- <b>Integers</b>: `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`, `u128`, `i128` (number of bits taken up in memory)
- <b>Floats</b>: `f32`, `f64`
- <b>Boolean</b>: `bool`
- <b>Chracters</b>: `char`
- <b>Tuples</b>: `()`
- <b>Arrays</b> (Arrays in Rust are <u>fixed-length</u>, Vectors are dynamic arrays)

---

## Development

Build:

```
cargo build
```

Execute:

```
cargo run
```

## Production

Build (for production):

```
cargo build --release
```

---
