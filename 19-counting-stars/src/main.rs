use std::io;

type Sky = Vec<Vec<char>>;

/// Reads a user input line.
fn read_line() -> String {
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Error: Unable to read user input.");
  input
}

fn main() {
  let mut itc = 0;
  loop {
    itc += 1;
    // If this is the 1000th test case, stop the program
    if itc > 1000 { println!("Invalid number of test cases."); break }
    // Get values
    let values: Vec<usize> = read_line().split(' ')
      .map(|s| s.trim().parse::<usize>())
      .filter_map(Result::ok).collect();
    // Check if the read data is invalid
    if values.len() != 2 { break; }
    let (r, c) = (values[0], values[1]);
    // Check if the read data is invalid
    if r > 101 || c > 101 { panic!("Invalid sky row or col.") }
    // Read the stars
    let mut sky: Sky = vec![vec!['.'; c]; r];
    for i in sky.iter_mut().take(r) {
      // Update sky
      *i = read_line().split("").map(|s| s.trim().parse::<char>())
        .filter_map(Result::ok).collect();
    }
    // Find the number of starts that can be seen in the sky
    let mut counter: usize = 0;
    for i in 0..r {
      for j in 0..c {
        // Check if this point of the sky has a star
        if sky[i][j] == '*' {
          // Vertex degree
          let mut degree: usize = 0;
          // Calculate vertex degree
          if i > 0 && sky[i - 1][j] == '*' { degree += 1; }
          if j > 0 && sky[i][j - 1] == '*' { degree += 1; }
          if i < r - 1 && sky[i + 1][j] == '*' { degree += 1; }
          if j < c - 1 && sky[i][j + 1] == '*' { degree += 1; }
          if i > 0 && j > 0 && sky[i - 1][j - 1] == '*' { degree += 1; }
          if i < r - 1 && j > 0 && sky[i + 1][j - 1] == '*' { degree += 1; }
          if i > 0 && j < c - 1 && sky[i - 1][j + 1] == '*' { degree += 1; }
          if i < r - 1 && j < c - 1 && sky[i + 1][j + 1] == '*' { degree += 1; }
          // If the degree is `0` it means that the star can be seen
          if degree == 0 { counter += 1; }
        }
      }
    }
    // Print solution
    println!("{}", counter);
  }
}
