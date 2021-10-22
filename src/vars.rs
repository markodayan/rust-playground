pub fn run() {
  let name = "Bob";

  // override immutability
  let mut age = 18;

  println!("My name is {} and I am {} but it is my birthday soon!", name, age);
  age = 19;
  println!("My name is {} and I am {}", name, age);

  // Define constant (have to specify type)
  const ID: i32 = 001;
  println!("ID: {}", ID);

  // Assign multiple vars (using tuple)
  let ( my_name, my_age ) = ("Alice" , 50);
  println!("{} is {} years old", my_name, my_age);
}