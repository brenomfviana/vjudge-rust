use std::io;
use std::collections::VecDeque;

const PRIME: usize = 131071;

/// Reads the whole input and returns a VecDeque of chars.
///
/// Why this is needed? Rust does not provide a `read_char` function and this
/// is the only way I find to read each char withou extern crates.
fn input() -> VecDeque<char> {
  // Vector to hold the whole stdio input
  let mut input: VecDeque<char> = VecDeque::new();
  loop {
    // Read a line
    let mut line = String::new();
    io::stdin().read_line(&mut line)
      .expect("Error: Unable to read user input.");
    // Check if there is no more lines to read
    if line.trim() == "" { break }
    // Convert the line into a vector of chars and add it to the input
    input.append(&mut line.chars().collect::<VecDeque<char>>());
  }
  // Return the input
  input
}

fn main() {
  // Read whole input
  let mut input = input();
  loop {
    // Read the binary number
    let mut binary: Vec<String> = vec![];
    while let Some(c) = input.pop_front() {
      // Add valid chars to the binary string
      if c == '0' || c == '1' { binary.push(format!("{}", c)); }
      // The binary string was fully read
      if c == '#' { break }
    }
    // Check if the end condition was reached
    if binary.is_empty() { break }
    // Convert the read binary into a unsigned number
    let mut num = 0;
    let binary: Vec<usize> = binary.iter()
      .map(|s| s.parse::<usize>())
      .filter_map(Result::ok).collect();
    for &c in &binary { num = (num << 1) + c; }
    // Check if num is bigger than
    if num >= PRIME { num %= PRIME; }
    // Print solution
    println!("{}", if num != 0 { "NO" } else { "YES" });
  }
}
