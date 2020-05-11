use std::io;
use std::cmp;

fn main() {
  // Index of the current test cases
  let mut i = 0;
  // Start program
  loop {
    // Get number of bachelors and spinsters
    let mut input = String::new();
    io::stdin().read_line(&mut input)
      .expect("Error: Unable to read user input.");
    let nums: Vec<String> = input.split(" ")
      .map(|s| s.trim().to_string()).collect();
    // Check if the number of bachelors and spinsters are zero
    if nums[0] == "0" && nums[1] == "0" {
      break;
    }
    // Set minimun age
    let mut minlbage = 61;
    // Convert values
    if let (Ok(nb), Ok(ns)) =
      (nums[0].parse::<usize>(), nums[1].parse::<usize>()) {
        // Read bachelors and spinsters
        for _ in 0..nb {
          let mut input = String::new();
          io::stdin().read_line(&mut input)
            .expect("Error: Unable to read user input.");
          let value = input.trim().parse::<usize>();
          if let Ok(value) = value {
            minlbage = cmp::min(minlbage, value);
          }
        }
        for _ in 0..ns {
          let mut input = String::new();
          io::stdin().read_line(&mut input)
            .expect("Error: Unable to read user input.");
        }
        // Calculate result
        // Once we just need to print the number of bachelors left and the
        // youngest of them we can just print zero, if number of bachelors is
        // less than the the number of spinsters, print the subtraction of the
        // number of bachelors by the the number of spinsters.
        if nb <= ns {
          println!("Case {}: {}", i + 1, 0);
        } else {
          println!("Case {}: {} {}", i + 1, nb - ns, minlbage);
        }
    }
    i += 1;
  }
}
