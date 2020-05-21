use std::io;
use std::collections::HashMap;

// Number of coins
const NOC: usize = 5;

// List of all coins
const COINS: &'static [u64] = &[1, 5, 10, 25, 50];

/// Finds the number of ways to receive `x` cents change.
fn solve(change: u64) -> u64 {
  // Cache for memoization
  let mut cache: HashMap<u64, u64> = HashMap::new();
  cache.insert(0, 1);
  // Calculate the ways to produce the change
  for i in 0..NOC {
    for j in COINS[i]..=change {
      // Insert the ways to produce the change `j`
      if let None = cache.get(&j) { cache.insert(j, cache[&(j - COINS[i])]); }
      // Update the ways to produce the change `j`
      else { cache.insert(j, cache[&j] + cache[&(j - COINS[i])]); }
    }
  }
  // Return the ways to produce the change
  cache[&change]
}

fn main() {
  loop {
    // Read the input
    let mut input = String::new();
    io::stdin().read_line(&mut input)
      .expect("Error: Unable to read user input.");
    // Check if the end condition was reached
    if input == "" { break; }
    // Check if the input is valid
    if let Ok(change) = input.trim().parse::<u64>() {
      // Find the number of ways to receive `x` cents change
      let ways = solve(change);
      // Print solution
      if ways == 1 { print!("There is only 1 way "); }
      else { print!("There are {} ways ", ways); }
      println!("to produce {} cents change.", change);
    }
  }
}
