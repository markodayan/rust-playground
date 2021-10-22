// {} = format specifier

pub fn run() {
  // Basic print
  println!("Hello World (from print.rs)");
  
  // Numerical
  println!("Number: {}", 24);
  println!("Number: {}", 24 + 1);
  
  // Positional Arguments
  println!("my {0} is your {0} and your {0} is mine", "House");
  println!("I write code in {0}, {1} and {2}, but my favourite is {1}", "JavaScript", "Rust", "C++");

  // Named Arguments
  println!("{name} likes to play {activity}", name = "Mark", activity = "Football");
  
  // Placeholder Traits
  println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

  // Placeholder for debug trait
  println!("{:?}", (12, true, "hello"));

  // Other random
  println!("My name is {}, king of {}", "Ozymandias", "kings");
  println!("My name is {} {} and I am {} years old", "Jon".to_owned() + "athan", "Doe", 50);
}