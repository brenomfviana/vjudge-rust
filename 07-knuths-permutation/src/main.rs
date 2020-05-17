use std::io;

/// Finds and prints all possible permutations.
fn knuth(string: &Vec<char>, kstr: &mut Vec<char>, index: usize) {
  // Check if the index has reached the string length
  if index == kstr.len() {
    // Print result
    for i in kstr { print!("{}", i); }
    println!("");
    return;
  }
  // Push chars
  for i in (0..=index).rev() {
    if let Some(ni) = usize::checked_sub(i, 1) { kstr[i] = kstr[ni]; }
  }
  // Fix and apply permutations
  for i in 0..(index + 1) {
    // Fix permutation
    kstr[i] = string[index];
    // Find permutations
    knuth(string, kstr, index + 1);
    // Check string bound and pull chars
    if i + 1 < string.len() { kstr[i] = kstr[i + 1]; }
  }
}

fn main() {
  let mut ntc = 0;
  loop {
    // Get string
    let mut input = String::new();
    io::stdin().read_line(&mut input)
      .expect("Error: Unable to read user input.");
    let input = input.trim().to_string();
    // Check if the string is empty and break loop
    if input.is_empty() { break; }
    // Separate results
    if ntc != 0 { println!(""); }
    // Find permutations
    let mut string: Vec<char> = input.chars().collect();
    knuth(&string.clone(), &mut string, 0);
    // Next test case
    ntc += 1;
  }
}
