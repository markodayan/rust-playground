pub fn run() {
  let name = "Bob";
  let mut age = 18;
  println!("My name is {} and I am {} but it is my birthday soon!", name, age);

  age = 19;

  println!("My name is {} and I am {}", name, age);

  // Define constant
  const ID: i32 = 001;
  println!("ID: {}", ID);
}