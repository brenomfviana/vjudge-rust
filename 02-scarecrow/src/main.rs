use std::io;

fn main() {
  // Get number of test cases
  let mut input = String::new();
  io::stdin().read_line(&mut input)
    .expect("Error: Unable to read user input.");
  let ntc = input.trim().parse::<usize>();
  if let Ok(ntc) = ntc {
    // Run test cases
    for i in 0..ntc {
      // Get field size
      let mut input = String::new();
      io::stdin().read_line(&mut input)
        .expect("Error: Unable to read user input.");
      let fs = input.trim().parse::<usize>();
      if let Ok(fs) = fs {
        // Read field
        let mut field = String::with_capacity(fs);
        io::stdin().read_line(&mut field)
          .expect("Error: Unable to read user input.");
        let field: Vec<char> = field.trim().to_string().chars().collect();
        // Calculate the number of scarecrows
        // We need to place scarecrows on '.' fields, and they protect three
        // fields: the field where it is placed, the left and right of it. So,
        // we can skip three fields always we place a scarecrow.
        let (mut cnt, mut j) = (0, 0);
        while j < fs {
          if field[j] == '.' {
            cnt += 1;
            j += 3;
          } else {
            j += 1;
          }
        }
        // Print solution
        println!("Case {}: {}", i + 1, cnt);
      } else {
        break;
      }
    }
  }
}
