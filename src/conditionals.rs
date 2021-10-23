pub fn run() {
  let age: u8 = 24;
  let check_id: bool = false;
  let knows_person_of_age = true;

  // if/else
  if age >= 21 && check_id || knows_person_of_age {
    println!("Allowed to enter");
  } else if age < 21 && check_id {
    println!("Not allowed to enter");
  } else {
    println!("Please show your ID");
  }

  // Shorthand if
  let is_of_age = if age >= 21 { true } else { false };
  println!("Is of age: {}", is_of_age);
}

