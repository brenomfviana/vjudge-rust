use std::io;

/// Reads a user input line.
fn read_line() -> String {
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Error: Unable to read user input.");
  input
}

fn main() {
  // Read the number of test cases
  let ntc = read_line().trim().parse::<usize>()
    .expect("Error: The given number of test cases is invalid.");
  // Check if the number of test cases is invalid
  if ntc > 100 { panic!("Invalid number of test cases.") }
  // Run test cases
  for i in 0..ntc {
    // Get field size
    let fs = read_line().trim().parse::<usize>()
      .expect("Error: The given field size is invalid.");
    // Check if the field size is invalid
    if fs >= 100 { panic!("Invalid field size.") }
    // Read field
    let field: Vec<char> = read_line().trim().to_string().chars().collect();
    // Ensure that the field has the right size
    assert_eq!(fs, field.len(), "The field has the wrong size.");
    // Calculate the number of scarecrows
    // We need to place scarecrows on '.' fields, and they protect three
    // fields: the field where it is placed, the left and right of it. So,
    // we can skip three fields always we place a scarecrow.
    let (mut cnt, mut j) = (0, 0);
    while j < fs { if field[j] == '.' { cnt += 1; j += 3; } else { j += 1; } }
    // Print solution
    println!("Case {}: {}", i + 1, cnt);
  }
}
