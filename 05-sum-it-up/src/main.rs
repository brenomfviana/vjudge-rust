use std::io;

/// Prints the equation that results in the sum.
fn print_sum(solution: &Vec<usize>) {
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
fn sum_it_up(goal: usize, numbers: Vec<usize>, solutions: &mut Vec<Vec<usize>>,
  mut solution: Vec<usize>) {
    // Check if the solution is valid
    if solution.iter().sum::<usize>() == goal &&
      !solutions.contains(&solution) {
        print_sum(&solution);
        solutions.push(solution.clone());
    }
    // Build solutions
    for (i, n) in numbers.iter().enumerate() {
      solution.push(*n);
      sum_it_up(goal, numbers[i + 1..].to_vec(), solutions, solution.clone());
      solution.pop();
    }
}

fn main() {
  loop {
    // Get data
    let mut input = String::new();
    io::stdin().read_line(&mut input)
      .expect("Error: Unable to read user input.");
    let data: Vec<usize> = input.split(" ")
      .map(|s| s.trim().parse::<usize>())
      .filter_map(Result::ok).collect();
    // Check if the end condition is valid
    if data.len() == 2 && data[0] == 0 && data[1] == 0 { break; }
    // Get the goal number and the list size
    let (goal, _) = (data[0], data[1]);
    let numbers: Vec<usize> = data[2..].to_vec();
    println!("Sums of {}:", goal);
    // Fint combinations
    let mut solutions: Vec<Vec<usize>> = vec![];
    sum_it_up(goal, numbers, &mut solutions, vec![]);
    // Check if there are solutions
    if solutions.is_empty() { println!("NONE"); }
  }
}
