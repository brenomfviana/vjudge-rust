use std::io;

const MAX: usize = 101;

/// Returns the list of prime numbers from 2 to `upperbound`.
fn sieve(upperbound: usize) -> Vec<usize> {
  // Auxiliary array to define the prime numbers
  let mut is_prime = vec![true; MAX];
  // List of prime numbers
  let mut primes: Vec<usize> = vec![];
  // Find all prime numbers from 0 to `upperbound`
  for i in 2..=upperbound {
    // Check if `i` is prime
    if is_prime[i] {
      for j in ((i * i)..=upperbound).step_by(i) { is_prime[j] = false; }
      primes.push(i);
    }
  }
  // Return the list of prime numbers
  primes
}

/// Returns the list of occurences of all prime numbers between 0 and `num`.
fn solve(mut num: usize) -> Vec<usize> {
  // Get all primes from 0 to `num`
  let primes = sieve(num);
  // size_t of occurences of each prime number
  let mut occurence = vec![0usize; primes.len()];
  // Count primes occurence
  while num != 1 {
    let mut aux = num;
    let mut j = 0;
    while aux > 1 {
      while aux % primes[j] == 0 {
        aux /= primes[j];
        occurence[j] += 1;
      }
      j += 1;
    }
    num -= 1;
  }
  // Return the list of occurences of prime numbers
  occurence
}

fn main() {
  loop {
    // Get input
    let mut input = String::new();
    io::stdin().read_line(&mut input)
      .expect("Error: Unable to read user input.");
    // Check if the end condition was reached
    if input.trim() == "0" { break; }
    // Convert input into a number
    let num = input.trim().parse::<usize>()
      .expect("Error: The given number is invalid.");
    // Check if the number is invalid
    if num >= MAX { panic!("The given number is too big.") }
    // Print original number
    print!("{}! = ", num);
    for &n in &solve(num) { print!("{} ", n); }
    println!();
  }
}
