use std::io;
use std::cmp;

const MAX: usize = 200;
const CARS: usize = 2;

/// Returns true if the suitcases fit on all car trunks and false otherwise.
fn solve(weights: Vec<usize>, weight_sum: usize) -> bool {
  // Check if the sum of weights are multiple of the number of CARS
  if weight_sum % CARS != 0 || weight_sum > MAX { return false; }
  // Calculate the maximun weight of each car
  let max_suitcase = weight_sum / CARS;
  // Memory array
  let mut memory = vec![vec![0; max_suitcase + 1]; weights.len()];
  // Calculate the m
  for i in 1..weights.len() {
    for j in 0..=max_suitcase {
      // If the current suitcase weight does not fit on the car trunk
      if weights[i - 1] > j {
        // Ignore the current suitcase, and the current weight does not change
        memory[i][j] = memory[i - 1][j];
      } else {
        // Calculate the weight with the new suitcase
        let weight = weights[i - 1] + memory[i - 1][j - weights[i - 1]];
        // Get the max weight and update the current weight
        memory[i][j] = cmp::max(memory[i - 1][j], weight);
      }
    }
  }
  // Since half of suitcases weigh together `max_suitcase`, we can check if the
  // suitcases that we add to a car has the same total weight, and if this is
  // true, then it means that we can divide the suitcases between the cars
  return memory[weights.len() - 1][max_suitcase] == max_suitcase;
}

fn main() {
  // Read number of test cases
  let mut input = String::new();
  io::stdin().read_line(&mut input)
    .expect("Error: Unable to read user input.");
  let ntc = input.trim().parse::<isize>();
  // Check if the number of test cases was read
  if let Ok(mut ntc) = ntc {
    // Run test cases
    while ntc > 0 {
      // A vector to hold all the weights of each suitcases
      let mut input = String::new();
      io::stdin().read_line(&mut input)
        .expect("Error: Unable to read user input.");
      let weights: Vec<usize> = input.split(" ")
        .map(|s| s.trim().parse::<usize>())
        .filter_map(Result::ok).collect();
      // Get the sum of the weights
      let sum: usize = weights.iter().sum();
      // Solve and print the solution
      if solve(weights, sum) { println!("YES"); }
      else { println!("NO"); }
      // Next test case
      ntc -= 1;
    }
  }
}
