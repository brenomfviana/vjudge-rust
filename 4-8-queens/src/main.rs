use std::io;
use std::cmp;

/// Checks if the sub-board is valid for the 8 queens problem.
fn is_valid(row: &Vec<usize>, r: usize, c: usize) -> bool {
  // For each row
  for i in 0..r {
    // Convert values to i8
    let (r, c, i, ri) = (r as i8, c as i8, i as i8, row[i] as i8);
    // Check risks
    if c == ri || (r - i).abs() == (c - ri).abs() { return false; }
  }
  true
}

/// Finds the number of moves to turn the board valid for the 8 queens problem.
fn search(row: &mut Vec<usize>, queens: &Vec<usize>, r: usize) -> usize {
  // Check if the recursion has reached the depth 8
  if r == 8 { return 0; }
  // Move queens
  let mut min = std::usize::MAX;
  for i in 0..8 {
    // Ignore invalid permutations
    if is_valid(&row, r, i) {
      // Move queen
      row[r] = i;
      // Keep searching
      let value = search(row, queens, r + 1);
      if i == queens[r] || value == std::usize::MAX {
        min = cmp::min(min, value);
      } else {
        min = cmp::min(min, value + 1);
      }
    }
  }
  min
}

fn main() {
  // Control variables
  let (mut itc, mut ntc) = (0, 0);
  // Run test cases
  while ntc < 1000 {
    // Get the vertical position of each queen
    let mut input = String::new();
    io::stdin().read_line(&mut input)
      .expect("Error: Unable to read user input.");
    let auxvec: Vec<String> = input.split(" ")
      .map(|s| s.trim().to_string()).collect();
    // Check if the number of queens is not eight and break loop
    if auxvec.len() != 8 { break; }
    // Initialize queens vector
    let mut queens: Vec<usize> = vec![];
    for s in auxvec.iter() {
      if let Ok(s) = s.parse::<usize>() { queens.push(s); }
    }
    let mut row = queens.clone();
    itc += 1;
    // Find number of moves to find 8 queens solution
    let min = search(&mut row, &queens, 0);
    // Print result
    println!("Case {}: {}", itc, min);
    ntc += 1;
  }
}
