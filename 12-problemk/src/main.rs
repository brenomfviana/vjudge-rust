use std::io;

/// Reads a user input line.
fn read_line() -> String {
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Error: Unable to read user input.");
  input
}

fn main() {
  loop {
    // Read input
    let mut input = read_line();
    // Check if the end condition is valid
		if input == "0 Fuel consumption 0" { break; }
    // Initialize control variables
		let (mut fuel, mut tank) = (0f64, 0f64);
		let (mut dist, mut cnsmptn, mut leak): (f64, f64, f64) = (0f64, 0f64, 0f64);
    // Calcultes the -------
		loop {
      // Auxiliary variables
      let (mut n, mut c): (f64, f64) = (0f64, 0f64);
      // Get input values
      let values: Vec<String> = input.split(' ')
        .map(|s| s.trim().to_string()).collect();
      // Get info from values
      if values.len() <= 3 {
        n = values[0].parse::<f64>().unwrap();
      }
      if values.len() == 4 {
        n = values[0].parse::<f64>().unwrap();
        c = values[3].parse::<f64>().unwrap();
      }
      // Spend fuel
			fuel += leak * (n - dist);
			fuel += (cnsmptn / 100f64) * (n - dist);
      // Update tank fuel amount
			tank = fuel.max(tank);
      // Check if the goal was reached and stop
			if values[1] == "Goal" { break; }
      // Update fuel cnsmptn
			if values[1] == "Fuel" { cnsmptn = c; }
      // Add a new leak to the tank
			else if values[1] == "Leak" { leak += 1f64; }
      // Visit a mechanic and fix all leaks
			else if values[1] == "Mechanic" { leak = 0f64; }
      // Restore all tank (reset the spent fuel)
			else if values[1] == "Gas" { fuel = 0f64; }
      // Update the reached distance
			dist = n;
      // Reset current input and get next input
      input = read_line();
		}
    // Print solution
    println!("{:.3}", tank);
	}
}
