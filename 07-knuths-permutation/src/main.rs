use std::collections::HashSet;
use std::io;
use std::iter::FromIterator;

/// Reads a user input line.
fn read_line() -> String {
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Error: Unable to read user input.");
  input
}

/// Finds and prints all possible permutations.
fn knuth(string: &[char], kstr: &mut Vec<char>, index: usize) {
  // Check if the index has reached the string length
  if index == kstr.len() {
    // Print result
    for i in kstr { print!("{}", i); }
    println!();
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
  let mut is_first = true;
  loop {
    // Separate results
    if !is_first { println!(); } is_first = false;
    // Get string
    let input = read_line().trim().to_string();
    // Check if the string is empty and break loop
    if input.is_empty() { break; }
    // Separate characters
    let mut string: Vec<char> = input.chars().collect();
    // Check if the number of characters is invalid
    if string.len() > 10 { panic!("Invalid number of characters.") }
    // Turn list into a set
    let set_str: HashSet<char> = HashSet::from_iter(string.iter().cloned());
    // Check if the list of characters is invalid
    if set_str.len() != string.len() { panic!("Invalid list of characters.") }
    // Find permutations
    knuth(&string.clone(), &mut string, 0);
  }
}
