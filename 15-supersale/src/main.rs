use std::io;

const MAX: usize = 30;

/// Reads a user input line.
fn read_line() -> String {
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Error: Unable to read user input.");
  input
}

/// Calculates the greater sum of the price of the products by person capacity.
fn solve(prices: Vec<usize>, weights: Vec<usize>, capacity: Vec<usize>)
  -> usize {
    // Memory array highest prices by weight
    let mut memory: Vec<usize> = vec![0; MAX];
    // Calculate the lightest and most expensive product
    for i in 0..weights.len() {
      for j in (weights[i]..MAX).rev() {
        if memory[j] < memory[j - weights[i]] + prices[i] {
          memory[j] = memory[j - weights[i]] + prices[i];
        }
      }
    }
    // Calculate the most expensive set of products by family
    let mut max_price = 0;
    for i in 0..capacity.len() { max_price += memory[capacity[i]]; }
    // Return the max possible sum of the price of the products
    max_price
}

fn main() {
  // Read the number of test cases
  let ntc = read_line().trim().parse::<isize>()
    .expect("Error: The given number of test cases is invalid.");
  // Check if the number of test cases is invalid
  if ntc > 1000 { panic!("Invalid number of test cases.") }
  // Run test cases
  for _ in 0..ntc {
    // Two vectors to hold all the object prices and weights
    let mut prices: Vec<usize> = vec![];
    let mut weights: Vec<usize> = vec![];
    let mut capacity: Vec<usize> = vec![];
    // Read the number of products
    let npr = read_line().trim().parse::<usize>()
      .expect("Error: The given number of products is invalid.");
    // Check if the number of products is invalid
    if npr > 1000 { panic!("Invalid number of products.") }
    for _ in 0..npr {
      // Get product price and weight
      let pair: Vec<usize> = read_line().split(' ')
        .map(|s| s.trim().parse::<usize>())
        .filter_map(Result::ok).collect();
      // Validity the read data
      if pair.len() != 2 { continue; }
      // Check if the price of object is invalid
      if pair[0] > 100 { panic!("Invalid price of object.") }
      // Check if the weight of object is invalid
      if pair[1] > 30 { panic!("Invalid weight of object.") }
      // Add values
      prices.push(pair[0]);
      weights.push(pair[1]);
    }
    // Validity the read data
    if prices.len() != npr && weights.len() != npr { break; }
    // Read the number of people
    let npe = read_line().trim().parse::<usize>()
      .expect("Error: The given number of people is invalid.");
    // Check if the number of people is invalid
    if npe > 100 { panic!("Invalid number of people.") }
    for _ in 0..npe {
      // Get person capacity
      let pc = read_line().trim().parse::<usize>()
        .expect("Error: The given person capacity is invalid.");
      // Check if the person capacity is invalid
      if pc > 30 { panic!("Invalid person capacity.") }
      // Add capacity
      capacity.push(pc);
    }
    // Validity the read data
    if capacity.len() != npe { break; }
    // Solve and print the result
    println!("{}", solve(prices, weights, capacity));
  }
}
