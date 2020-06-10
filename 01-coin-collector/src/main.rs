use std::io;

/// Reads a user input line.
fn read_line() -> String {
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Error: Unable to read user input.");
  input
}

/// Finds and prints the amount of coins that can be withdrawn.
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
    if coins[i + 1] > amount + c { amount += c; cnt += 1; }
  }
  println!("{}", cnt);
}

/// Read the coins.
fn read_coins(size: usize) -> Vec<usize> {
  // Get list of coins
  let coins: Vec<usize> = read_line().split(' ')
    .map(|s| s.trim().parse::<usize>())
    .filter_map(Result::ok).collect();
  // If the number of coins is greater than the size then return an empty list
  if coins.len() > size { panic!("Invalid list of coins.") }
  // Check if the coin with the highest value is valid
  if coins.iter().max() >= Some(&1000000000) {
    panic!("Invalid list of coins.")
  }
  // Return the list of coins
  coins
}

fn main() {
  // Read the number of test cases
  let ntc = read_line().trim().parse::<usize>()
    .expect("Error: The given number of test cases is invalid.");
  // Check if the number of test cases is invalid
  if ntc > 1000 { panic!("Invalid number of test cases.") }
  // Run test cases
  for _ in 0..ntc {
    // Read the number of coins
    let nc = read_line().trim().parse::<usize>()
      .expect("Error: The given number of coins is invalid.");
    // Find the maximum number of coins collected from a single withdrawal
    withdraw(read_coins(nc));
  }
}
