pub fn run() {
  // Immutable, fixed-length
  let _fixed = "Hello";

  // Growable (heap-allocated data structure)
  let mut hello = String::from("Hello ");  
  // Get length
  println!("Length: {}", hello.len());

  // Push char (you can only push to growable string)
  hello.push('W');

  // Push string (you can only push to growable string)
  hello.push_str("orld!");
  println!("{}", hello);

  // Capacity in bytes ("Hello World!" is 12 bytes - one for each char)
  println!("Capacity: {}", hello.capacity());
  println!("Is Empty: {}", hello.is_empty());
  
  // Check if contains sub-string
  println!("Contains 'World': {}", hello.contains("World"));

  // Replace (appears non-destructive)
  println!("Replace: {}", hello.replace("World", "Planet"));

  // Loop through string by whitespace
  for word in hello.split_whitespace() {
    println!("{}", word);
  }

  // Create string with capacity (note we make a growable but assign a memory allocation limit)
  let mut s = String::with_capacity(10);
  s.push('a');
  s.push('b');
  println!("{}", s);

  // Assertion testing (does nothing if passed, shows assertion failure if not satisfied)
  assert_eq!(2, s.len());
  assert_eq!(10, s.capacity());
  

  // let x1: &str = "hello";
  // let x2 = "world";
  // println!("{} {}", x1, x2);
}
