use std::io;

/// Find and print the amount of coins that can be withdrawn.
///
/// Algorithm explanation
/// We only can ensure this for a ascending sorted list of coins.
/// 1. if the next coin amount is greater than the current amount then we can
///   ensure that for the current amount the current coin can be withdrawn in
///   a single withdrawal.
/// 2. or else we can ensure that for the current amount the current coin and
///   the next can be withdrawn in a single withdrawal.
/// 3. We can count always with the last coin because we can ensure we always
///   can withdraw it according to the previous poins.
fn withdraw(coins : Vec<usize>) {
  let (mut cnt, mut amount) = (1, 0);
  for (i, c) in coins[..(coins.len() - 1)].iter().enumerate() {
    if coins[i + 1] > amount + c {
      amount += c;
      cnt += 1;
    }
  }
  println!("{}", cnt);
}

fn read_coins(size: usize) -> Vec<usize> {
  // Get list of coins
  let mut input = String::new();
  io::stdin().read_line(&mut input)
    .expect("Error: Unable to read user input.");
  input = input.trim().to_string();
  // Split the coins line
  let split_coins: Vec<String> = input.split(" ")
    .map(|s| s.to_string()).collect();
  // Convert coin value to unsingned integer
  let mut coins: Vec<usize> = vec![];
  for v in &split_coins {
    if let Ok(v) = v.parse::<usize>() {
      coins.push(v);
    }
  }
  // Check if the value is greater then the number of couples
  if coins.len() > size {
    // Return an invalid list
    return vec![];
  }
  coins
}

fn main() {
  // Get number of test cases
  let mut input = String::new();
  io::stdin().read_line(&mut input)
    .expect("Error: Unable to read user input.");
  let ntc = input.trim().parse::<usize>();
  if let Ok(mut ntc) = ntc {
    // Run test cases
    while ntc > 0 {
      // Get number of coins
      let mut input = String::new();
      io::stdin().read_line(&mut input)
        .expect("Error: Unable to read user input.");
      let nc = input.trim().parse::<usize>();
      if let Ok(nc) = nc {
        // Read the coins
        let coins = read_coins(nc);
        // Find the maximum number of coins collected from a single withdrawal
        withdraw(coins);
        // Next test case
        ntc -= 1;
      } else {
        break;
      }
    }
  }
}
