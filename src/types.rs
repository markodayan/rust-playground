pub fn run() {
  // 'i32' by default (inference)
  let x = 1;

  // 'f64' by default (inference)
  let y = 2.5;

  // explicit type definition
  let z: i64 = 45000;

  // Find max size
  println!("Max i32: {}", std::i32::MAX);
  println!("Max i64: {}", std::i64::MAX);

  // Boolean
  let is_active = true;
  let not_active: bool = false;

  // Get boolean from expression
  let is_greater: bool = 10 > 5;
  let is_less: bool = 10 < 5;

  // Character (you have to use single-quotes to represent chars)
  let a1 = 'a';
  let peach = '\u{1F351}';

  println!(
    "{:?}",
    (x, y, z, is_active, not_active, is_greater, is_less, a1, peach)
  );
}
