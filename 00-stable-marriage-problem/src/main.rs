use std::io;

/// Applies Gale-Shapley's algorithm
fn gale_shapley(mp: Vec<Vec<usize>>, wp: Vec<Vec<usize>>, nu: usize) {
  // Init couples
  let mut couples: Vec<Option<usize>> = vec![None; nu];
  // Find solution
  while couples.contains(&None) {
    // Get next man
    let mi = couples.iter().position(|&e| e == None);
    if let Some(mi) = mi {
      // Check each of him preferences
      if let Some(mip) = mp.get(mi) {
        for wi in mip {
          // Check if the woman is available
          if !couples.contains(&Some(*wi)) {
            couples[mi] = Some(*wi);
            break;
          } else {
            // Get current fiance
            let c = couples.iter().position(|&e| e == Some(*wi));
            if let Some(c) = c {
              if let Some(wip) = wp.get(*wi) {
                let a = wip.iter().position(|&e| e == mi);
                let b = wip.iter().position(|&e| e == c);
                // Check if the woman prefers the new man and update marriage
                if let (Some(a), Some(b)) = (a, b) {
                  if a < b {
                    couples[mi] = Some(*wi);
                    couples[c] = None;
                    break;
                  }
                }
              }
            }
          }
        }
      }
    }
  }
  // Print solution
  for (i, c) in couples.iter().enumerate() {
    println!("{} {}", i + 1, c.unwrap() + 1);
  }
}

/// Gets the person's preferences
fn get_preferences(size: usize) -> Vec<Vec<usize>> {
  // Init preferences list
  let mut preferences: Vec<Vec<usize>> = vec![];
  // For each partner
  for _ in 0..size {
    // Get preferences
    let mut input = String::new();
    io::stdin().read_line(&mut input)
      .expect("Error: Unable to read user input.");
    // Split the preferences line
    let split_prefs: Vec<String> = input.split(" ")
      .map(|s| s.to_string()).collect();
    // Convert values to unsingned integer
    let mut values: Vec<usize> = vec![];
    for v in &split_prefs[1..] {
      if let Ok(v) = v.parse::<usize>() {
        let v = v - 1;
        // If `v` is greater than the number of couples return an empty list
        if v >= size { return vec![]; }
        values.push(v);
      }
    }
    // Add to preferences
    preferences.push(values);
  }
  return preferences;
}

fn main() {
  // Get number of test cases
  let mut input = String::new();
  io::stdin().read_line(&mut input)
    .expect("Error: Unable to read user input.");
  let ntc = input.trim().parse::<usize>();
  // Check if the number of test cases was read
  if let Ok(mut ntc) = ntc {
    // Run test cases
    while ntc > 0 {
      // Get number of couples
      let mut input = String::new();
      io::stdin().read_line(&mut input)
        .expect("Error: Unable to read user input.");
      let nc = input.trim().parse::<usize>();
      // Check if the number of couples was read
      if let Ok(nc) = nc {
        // Read men and women preferences
        let wp = get_preferences(nc);
        let mp = get_preferences(nc);
        // Check if the preferences are not valid
        if mp.is_empty() || wp.is_empty() || mp.len() != nc || wp.len() != nc {
          break;
        }
        // Solve stable marriage problem
        gale_shapley(mp, wp, nc);
        // Next test case
        ntc -= 1;
      } else { break; }
    }
  }
}
