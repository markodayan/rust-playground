use std::mem;

pub fn run() {
  let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

  // Res-assign value
  numbers[2] = 20;

  // Add onto vector
  numbers.push(6);
  numbers.push(7);

  // Pop off last value
  numbers.pop();

  // print entire array (using debug trait)
  println!("{:?}", numbers);

  // Get single val
  println!("Single Value: {}", numbers[0]);

  // Get vector length
  println!("Vector Length: {}", numbers.len());

  // Vectors are stack allocated (like arrays)
  println!("Vector occupies {} bytes ", mem::size_of_val(&numbers));

  // Get slice (return including starting to not-including index)
  let slice: &[i32] = &numbers[0..2]; // returns [1, 2] vector
  println!("Slice: {:?}", slice);

  // Loop through vector values
  for x in numbers.iter() {
    println!("Number: {}", x);
  }

  // Loop and mutate values
  for x in numbers.iter_mut() {
    *x *= 2;
  }

  println!("Numbers Vec: {:?}", numbers);
}