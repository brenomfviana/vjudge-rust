use std::io;

/// Reads a user input line.
fn read_line() -> String {
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Error: Unable to read user input.");
  input
}

/// Finds all possible paths of a maze.
fn find_path(maze: &mut Vec<&str>, size: usize, p: usize, e: usize,
  dir: String) {
    // Convert positions to a 2D position
    let (x, y) = (p % size, (p / size) as usize);
    // Set room as visited
    maze[y * size + x] = "2";
    // Check if the end was reached and print the result
    if p == e { print!("{} ", dir); }
    else {
      let up = (y as isize - 1) >= 0;
      let ny = (y as isize - 1) * size as isize + x as isize;
      if up && maze[ny as usize] == "1" {
        let next = (y - 1) * size + x;
        find_path(&mut maze.clone(), size, next, e, format!("{}{}", dir, "U"));
      }
      if y + 1 < size && maze[(y + 1) * size + x] == "1" {
        let next = (y + 1) * size + x;
        find_path(&mut maze.clone(), size, next, e, format!("{}{}", dir, "D"));
      }
      let left = (x as isize - 1) >= 0;
      let ny = y as isize * size as isize + x as isize - 1;
      if left && maze[ny as usize] == "1" {
        let next = y * size + (x - 1);
        find_path(&mut maze.clone(), size, next, e, format!("{}{}", dir, "L"));
      }
      if x + 1 < size && maze[y * size + x + 1] == "1" {
        let next = y * size + (x + 1);
        find_path(&mut maze.clone(), size, next, e, format!("{}{}", dir, "R"));
      }
    }
}

fn main() {
  // Read the number of test cases
  let ntc = read_line().trim().parse::<usize>()
    .expect("Error: The given number of test cases is invalid.");
  // Check if the number of test cases is invalid
  if ntc > 10 { panic!("Invalid number of test cases.") }
  // Run test cases
  for _ in 0..ntc {
    // Read maze size
    let msize = read_line().trim().parse::<usize>()
      .expect("Invalid maze size.");
    // Check if the maze size is invalid
    if msize < 2 && msize > 10 { panic!("Invalid maze size.") }
    // Read maze
    let data = read_line();
    let mut maze: Vec<&str> = data.split(' ').map(|s| s.trim()).collect();
    // Find and print all solutions of a maze
    find_path(&mut maze, msize, 0, msize * msize - 1, String::new());
    println!();
  }
}
