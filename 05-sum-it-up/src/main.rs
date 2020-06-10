use std::io;

/// Reads a user input line.
fn read_line() -> String {
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Error: Unable to read user input.");
  input
}

/// Prints the equation that results in the sum.
fn print_sum(solution: &[usize]) {
  // If the solution has only one number
  if solution.len() == 1 { println!("{}", solution[0]); }
  else {
    for (i, n) in solution.iter().enumerate() {
      // Add the first numbers
      if i != solution.len() - 1 { print!("{}+", n); }
      // Add the last number
      else { println!("{}", n); }
    }
  }
}

/// Finds the equations that results in the sum (`goal`).
fn sum_it_up(goal: usize, numbers: Vec<usize>, sltns: &mut Vec<Vec<usize>>,
  mut sltn: Vec<usize>) {
    // Check if the solution is valid
    if sltn.iter().sum::<usize>() == goal && !sltns.contains(&sltn) {
      print_sum(&sltn);
      sltns.push(sltn.clone());
    }
    // Build solutions
    for (i, n) in numbers.iter().enumerate() {
      sltn.push(*n);
      sum_it_up(goal, numbers[i + 1..].to_vec(), sltns, sltn.clone());
      sltn.pop();
    }
}

fn main() {
  loop {
    // Read the data
    let data: Vec<usize> = read_line().split(' ')
      .map(|s| s.trim().parse::<usize>())
      .filter_map(Result::ok).collect();
    // Check if the end condition is valid
    if data.len() == 2 && data[0] == 0 && data[1] == 0 { break; }
    // Get the goal number and the list size
    let (goal, _) = (data[0], data[1]);
    // Check if the goal is invalid
    if goal >= 1000 { panic!("Invalid goal.") }
    // Get the numbers
    let numbers: Vec<usize> = data[2..].to_vec();
    // Check if the number with the highest value is valid
    if numbers.iter().max() >= Some(&100) { panic!("Invalid list of numbers.") }
    // Fint combinations
    let mut solutions: Vec<Vec<usize>> = vec![];
    println!("Sums of {}:", goal);
    sum_it_up(goal, numbers, &mut solutions, vec![]);
    // Check if there are solutions
    if solutions.is_empty() { println!("NONE"); }
  }
}
