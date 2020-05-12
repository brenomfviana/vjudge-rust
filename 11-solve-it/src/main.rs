use std::io;

const ERR: f64 = 0.000000001;

fn f(params: &Vec<f64>, x: f64) -> f64 {
  // Get parameters
  let (p, q, r) = (params[0], params[1], params[2] as f64);
  let (s, t, u) = (params[3], params[4], params[5] as f64);
  // Calculate function
  return p * (-x).exp() + q * x.sin() + r * x.cos() +
    s * x.tan() + t * x.powi(2) + u;
}

fn pass(params: &Vec<f64>, x: f64) -> f64 {
  // Get parameters
  let (p, q, r) = (params[0], params[1], params[2] as f64);
  let (s, t, _) = (params[3], params[4], params[5] as f64);
  // Calculate function
  return - p * (-x).exp() + q * x.cos() - r * x.sin() +
    s / x.cos() / x.cos() + t * x * 2f64;
}

fn solve(params: &Vec<f64>, mut x: f64) -> f64 {
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
    let ps: Vec<f64> = input.split(" ")
      .map(|s| s.trim().to_string().parse::<f64>())
      .filter_map(Result::ok).collect();
    // Check if the function can be solver with the given parameters
    if f(&ps, 1f64) * f(&ps, 0f64) > 0f64 { println!("No solution"); continue; }
    if ERR > f(&ps, 0f64).abs() { println!("0.0000"); }
    if ERR > f(&ps, 1f64).abs() { println!("1.0000"); }
    // Run solver and print result
    println!("{:.4}", solve(&ps, 1f64));
  }
}
