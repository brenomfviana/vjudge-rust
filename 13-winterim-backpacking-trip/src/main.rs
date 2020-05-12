use std::io;

/// Checks if `x` is the best cost for the trip.
fn is_best_cost(campsites: &Vec<usize>, nnt: usize, x: usize) -> bool {
  // Control variables
  let mut i: usize = 0;
  // Stop counter
  let mut counter: usize = 0;
  // Stop cost
  let mut cost: usize = 0;
  // Check if `x` is the best cost
  while i < campsites.len() {
    // If `x` is greater or equal than the current cost plus the next campsite
    // cost or `x` is less than the next campsite cost, then we can visit the
    // next campsite
    if x >= cost + campsites[i] || x < campsites[i] {
      // Visit a new campsite and increase stop cost and check next campsite
      cost += campsites[i]; i += 1;
      // If the current cost is greater than `x` then `x` is not the higher cost
      if x < cost { return false; }
    } else {
      // Add a stop
      counter += 1;
      // check if the number of stops is greater than the number of nights
      // (it is invalid)
      if counter > nnt { return false; }
      // Reset the stop cost
      cost = 0;
    }
  }
  return counter <= nnt;
}

/// Runs a division and conquer approach to find the solution.
fn solve(campsites: &Vec<usize>, nnt: usize, total_distance: usize) -> usize {
  // Get low and high
  let (mut low, mut high): (usize, usize) = (0, total_distance);
  // Find best solution
  while low <= high {
    // Calculate middle
    let mid = (high + low) / 2;
    // Update `low` or `high`
    if is_best_cost(campsites, nnt, mid) { high = mid - 1; }
    else { low = mid + 1; }
  }
  // Check which of `low` and `high` is the best cost and return it
  if is_best_cost(campsites, nnt, low) { return low; }
  return high;
}

fn main() {
  loop {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input)
      .expect("Error: Unable to read user input.");
    // Check if the end condition was reached
    if input == "" { break; }
    // Get the number of campsites and the number of nights of the trip
    let params: Vec<usize> = input.split(" ")
      .map(|s| s.trim().to_string().parse::<usize>())
      .filter_map(Result::ok).collect();
    let (nc, nnt) = (params[0], params[1]);
    // Read campsites distances
    let mut campsites: Vec<usize> = vec![];
    let mut total_distance: usize = 0;
    while nc >= campsites.len() {
      // Read input
      let mut input = String::new();
      io::stdin().read_line(&mut input)
        .expect("Error: Unable to read user input.");
      let distance: usize = input.trim().to_string().parse::<usize>().unwrap();
      campsites.push(distance);
      total_distance += distance;
    }
    // Find the best trip planning and print it
    println!("{}", solve(&campsites, nnt, total_distance));
    // Ignore the empty line between test cases
    let mut input = String::new();
    io::stdin().read_line(&mut input)
      .expect("Error: Unable to read user input.");
  }
}
