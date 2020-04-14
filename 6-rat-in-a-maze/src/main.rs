use std::io;

/// Finds all posible paths of a maze.
fn find_path(maze: &mut Vec<&str>, size: usize, p: usize, e: usize,
  dir: String) {
    // Convert positions to a 2D position
    let (x, y) = (p % size, (p / size) as usize);
    // Set room as visited
    maze[y * size + x] = "2";
    // Check if the end was reached
    if p == e {
      // Print result
      print!("{} ", dir);
    } else {
      let up = (y as isize - 1) >= 0;
      let ny = (y as isize - 1) * size as isize + x as isize;
      if up && maze[ny as usize] == "1" {
        let next = (y - 1) * size + x;
        find_path(&mut maze.clone(), size, next, e, dir.clone() + "U");
      }
      if y + 1 < size && maze[(y + 1) * size + x] == "1" {
        let next = (y + 1) * size + x;
        find_path(&mut maze.clone(), size, next, e, dir.clone() + "D");
      }
      let left = (x as isize - 1) >= 0;
      let ny = y as isize * size as isize + x as isize - 1;
      if left && maze[ny as usize] == "1" {
        let next = y * size + (x - 1);
        find_path(&mut maze.clone(), size, next, e, dir.clone() + "L");
      }
      if x + 1 < size && maze[y * size + x + 1] == "1" {
        let next = y * size + (x + 1);
        find_path(&mut maze.clone(), size, next, e, dir.clone() + "R");
      }
    }
}

fn main() {
  // Read number of test cases
  let mut input = String::new();
  io::stdin().read_line(&mut input)
    .expect("Error: Unable to read user input.");
  let mut itc = input.trim().to_string().parse::<usize>().unwrap();
  // Run test cases
  while itc > 0 {
    // Read maze size
    let mut input = String::new();
    io::stdin().read_line(&mut input)
      .expect("Error: Unable to read user input.");
    let msize = input.trim().to_string().parse::<usize>().unwrap();
    // Read maze
    let mut input = String::new();
    io::stdin().read_line(&mut input)
      .expect("Error: Unable to read user input.");
    let mut maze: Vec<&str> = input.split(" ").map(|s| s.trim()).collect();
    // Find and print all solutions of a maze
    find_path(&mut maze, msize, 0, msize * msize - 1, String::new());
    print!("\n");
    // Next test case
    itc -= 1;
  }
}
