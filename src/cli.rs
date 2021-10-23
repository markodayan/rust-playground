use std::env;

pub fn run() {
  let args: Vec<String> = env::args().collect();
  let name = "John";
  let status = "100%";

  // get second arg eg. hello from "cargo run hello"
  let command = args[1].clone();
  println!("Command: {}", command);
  println!("Args: {:?}", args);

  if command == "hello" {
    println!("Hi {}, how are you?", name);
  } else if command == "status" {
    println!("Status is {}", status);
  } else {
    println!("Invalid Command 🐶");
  }

}

// cargo run -> ["target/debug/rust_practice"]
// cargo run hello -> ["target/debug/rust_practice", "hello"]
// cargo run hello world -> ["target/debug/rust_practice", "hello", "world"]