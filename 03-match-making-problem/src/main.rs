use std::io;
use std::cmp;

/// Reads a user input line.
fn read_line() -> String {
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Error: Unable to read user input.");
  input
}

fn main() {
  // Index of the current test cases
  let mut i = 0;
  // Start program
  loop {
    // Get number of bachelors and spinsters
    let nm: Vec<String> = read_line().split(" ")
      .map(|s| s.trim().to_string()).collect();
    // Convert values
    let nb = nm[0].parse::<usize>()
      .expect("Error: The given number of bachelors is invalid.");
    let ns = nm[1].parse::<usize>()
      .expect("Error: The given number of spinsters is invalid.");
    // Check if the number of bachelors and spinsters are valid
    if nb >= 10000 && ns >= 10000 {
      panic!("Invalid number of bachelors and spinsters.")
    }
    // Set minimun age
    let mut minlbage = 61;
    // Read bachelors
    for _ in 0..nb {
      let value = read_line().trim().parse::<usize>();
      // Find the age of the youngest bachelor
      if let Ok(value) = value { minlbage = cmp::min(minlbage, value); }
    }
    // Read spinsters
    for _ in 0..ns { read_line(); }
    // Calculate result
    // Once we just need to print the number of bachelors left and the
    // youngest of them we can just print zero, if number of bachelors is
    // less than the the number of spinsters, print the subtraction of the
    // number of bachelors by the the number of spinsters.
    if nb <= ns { println!("Case {}: {}", i + 1, 0); }
    else { println!("Case {}: {} {}", i + 1, nb - ns, minlbage); }
    i += 1;
  }
}
