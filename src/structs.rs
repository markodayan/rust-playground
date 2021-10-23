// Structs - used to create custom data types

// Traditional Struct
struct Color {
  red: u8,
  green: u8,
  blue: u8
}

// Tuple Struct
struct Colour(u8, u8, u8);


struct Person {
  first_name: String,
  last_name: String
}

impl Person {
  // Construct person
  fn new(first: &str, last: &str) -> Person {
    Person {
      first_name: first.to_string(),
      last_name: last.to_string()
    }
  }

  // Get full name (no semi colon since we returning this)
  fn full_name(&self) -> String {
    format!("{} {}", self.first_name, self.last_name)
  }

  // Set last name
  fn set_last_name(&mut self, last: &str) {
    self.last_name = last.to_string();
  }

  // Name to tuple (no semi colon since we returning this)
  fn to_tuple(self) -> (String, String) {
    (self.first_name, self.last_name)
  }
}

pub fn run() {
  // Using Traditional Struct
  let mut c = Color {
    red: 255,
    green: 0,
    blue: 0
  };

  c.green = 69;
  println!("Color: {} {} {}", c.red, c.green, c.blue);

  // Using Tuple Struct
  let mut tc = Colour(255, 0, 0);
  tc.2 = 12;
  println!("Colour: {} {} {}", tc.0, tc.1, tc.2);


  // Using impl struct 
  let mut p = Person::new("John", "Doe");
  println!("Person {} {}", p.first_name, p.last_name);
  println!("Full name: {}", p.full_name());
  p.set_last_name("Smith");
  println!("Full name: {}", p.full_name());
  println!("Person Tuple: {:?}", p.to_tuple());
}