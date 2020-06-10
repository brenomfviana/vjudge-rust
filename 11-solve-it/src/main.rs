use std::io;

const ERR: f64 = 0.000000001;

/// Defines function to be solved.
#[allow(clippy::many_single_char_names)]
fn f(params: &[f64], x: f64) -> f64 {
  // Get parameters
  let (p, q, r) = (params[0], params[1], params[2] as f64);
  let (s, t, u) = (params[3], params[4], params[5] as f64);
  // Calculate function
  p * (-x).exp() + q * x.sin() + r * x.cos() + s * x.tan() + t * x.powi(2) + u
}

/// Defines function to for passing.
#[allow(clippy::many_single_char_names)]
fn pass(params: &[f64], x: f64) -> f64 {
  // Get parameters
  let (p, q, r) = (params[0], params[1], params[2] as f64);
  let (s, t, _) = (params[3], params[4], params[5] as f64);
  // Calculate function
  - p * (-x).exp() + q * x.cos() - r * x.sin() +
    s / x.cos() / x.cos() + t * x * 2f64
}

/// Solves the function `f`.
fn solve(params: &[f64], mut x: f64) -> f64 {
  loop {
    // Calculate `f(x)`
    let result = f(params, x);
    // Check if the result was found
    if ERR > result.abs() { return x; }
    // Update `x`
    x = x - result / pass(params, x);
  }
}

fn main() {
  loop {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input)
      .expect("Error: Unable to read user input.");
    // Check if the end condition was reached
    if input == "" { break; }
    // Convert input to an array of integers (the function parameters)
    let ps: Vec<f64> = input.split(' ')
      .map(|s| s.trim().parse::<f64>())
      .filter_map(Result::ok).collect();
    // The list of parameters must have 6 elements
    assert_eq!(ps.len(), 6, "Invalid number of parameters.");
    // Check if the function can be solver with the given parameters
    if f(&ps, 1f64) * f(&ps, 0f64) > 0f64 { println!("No solution"); continue; }
    if ERR > f(&ps, 0f64).abs() { println!("0.0000"); }
    if ERR > f(&ps, 1f64).abs() { println!("1.0000"); }
    // Run solver and print result
    println!("{:.4}", solve(&ps, 1f64));
  }
}
