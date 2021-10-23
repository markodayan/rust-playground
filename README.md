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
- Vectors are re-sizable arrays
- Conditionals are used to check the condition of something and act on the result (analogous to if statements)
- Loops are used to iterate until a condtion is met
- Reference pointers point to a resource in memory
- With <b>non-primitives</b>, if you assign another variale to a piece of data, <u>the first variable will no longer hold that value</u>. You will need to use a reference (&) to point to the resource

```rust
  // This will fail - you need to instead make a reference
  let vec1 = vec![1,2,3];
  let vec2 = vec1;
  println!("Values: {:?}", (vec1, vec2));
```

Correct way (creating a reference):

```rust
  let vec1 = vec![1,2,3];
  let vec2 = &vec1;
  println!("Values: {:?}", (&vec1, vec2));
```

- Structs are ued to make custom data types
- Enums are types which have a few discrete definitive values

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

## Cargo Command Line Arguments
