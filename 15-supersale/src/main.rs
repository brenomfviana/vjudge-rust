use std::io;

const MAX: usize = 30;

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
  // Read number of test cases
  let mut input = String::new();
  io::stdin().read_line(&mut input)
    .expect("Error: Unable to read user input.");
  let ntc = input.trim().parse::<isize>();
  // Check if the number of test cases was read
  if let Ok(mut ntc) = ntc {
    // Run test cases
    while ntc > 0 {
      // Two vectors to hold all the object prices and weights
      let mut prices: Vec<usize> = vec![];
      let mut weights: Vec<usize> = vec![];
      let mut capacity: Vec<usize> = vec![];
      // Get number of products
      let mut input = String::new();
      io::stdin().read_line(&mut input)
        .expect("Error: Unable to read user input.");
      let npr = input.trim().parse::<usize>();
      // Check if the number of products was read
      if let Ok(npr) = npr {
        for _ in 0..npr {
          // Get product data
          let mut input = String::new();
          io::stdin().read_line(&mut input)
            .expect("Error: Unable to read user input.");
          // Get product price and weight
          let pair: Vec<usize> = input.split(" ")
            .map(|s| s.trim().parse::<usize>())
            .filter_map(Result::ok).collect();
          // Validity the read data
          if pair.len() != 2 { continue; }
          // Add values
          prices.push(pair[0]);
          weights.push(pair[1]);
        }
        // Validity the read data
        if prices.len() != npr && weights.len() != npr { break; }
      } else { break; }
      // Get number of people
      let mut input = String::new();
      io::stdin().read_line(&mut input)
        .expect("Error: Unable to read user input.");
      let npe = input.trim().parse::<usize>();
      // Check if the number of people was read
      if let Ok(npe) = npe {
        for _ in 0..npe {
          // Get person capacity
          let mut input = String::new();
          io::stdin().read_line(&mut input)
            .expect("Error: Unable to read user input.");
          let pe = input.trim().parse::<usize>();
          // Check if the capacity was read
          if let Ok(pe) = pe { capacity.push(pe); }
        }
        // Validity the read data
        if capacity.len() != npe { break; }
      } else { break; }
      // Solve and print the result
      println!("{}", solve(prices, weights, capacity));
      // Next test case
      ntc -= 1;
    }
  }
}
