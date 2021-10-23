use std::mem;

pub fn run() {
  // Create array of i32 integers (with length 5)
  let numbers: [i32; 5] = [1, 2, 3, 4, 5];
  // print entire array (using debug trait)
  println!("{:?}", numbers);
  // Get single val
  println!("Single Value: {}", numbers[0]);

  let mut numbers_m: [i32; 5] = [1, 2, 3, 4, 5];
  // Res-assign value
  numbers_m[2] = 20;
  // print entire array (using debug trait)
  println!("{:?}", numbers_m);
  // Get single val
  println!("Single Value: {}", numbers_m[0]);


  // Get array length
  println!("Array Length: {}", numbers.len());

  // Get memory occupied by array (in this case its 20 bytes) [each entry is 4 bytes]
  println!("Array occupies {} bytes ", mem::size_of_val(&numbers));

  // Get slice (return including starting to not-including index)
  let slice: &[i32] = &numbers[0..2]; // returns [1, 2] array
  println!("Slice: {:?}", slice);
}