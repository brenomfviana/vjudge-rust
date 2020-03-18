use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

// Define an alias for read lines from input file
type Lines = std::io::Lines<std::io::BufReader<File>>;
type BufResult = std::io::Result<io::Lines<io::BufReader<File>>>;
type U8Result = std::result::Result<usize, std::num::ParseIntError>;

/// Read an input file and returns its content
fn input(filename: String) -> BufResult {
  Ok(io::BufReader::new(File::open(filename)?).lines())
}

/// Get a line and convert its content to an integer
fn get_int(lines: &mut Lines) -> U8Result {
  // Check if the line is valid
  if let Some(Ok(line)) = lines.next() {
    return line.parse::<usize>();
  }
  // Return a parse error
  "-".parse::<usize>()
}

/// Get the person's preferences
fn get_preferences(lines: &mut Lines, size: usize) -> Vec<Vec<usize>> {
  // Init preferences list
  let mut preferences: Vec<Vec<usize>> = vec![];
  // For each partner
  for _ in 0..size {
    // Check if the line is valid
    if let Some(Ok(line)) = lines.next() {
      // Split the preferences line
      let split_prefs: Vec<String> = line.split(" ")
        .map(|s| s.to_string()).collect();
      // Convert values to unsingned integer
      let mut values: Vec<usize> = vec![];
      for v in &split_prefs[1..] {
        if let Ok(v) = v.parse::<usize>() {
          let v = v - 1;
          if v >= size {
            // Return an invalid list
            return vec![];
          }
          values.push(v);
        }
      }
      // Add to preferences
      preferences.push(values);
    }
  }
  return preferences;
}

/// Apply Gale-Shapley's algorithm
fn gale_shapley(mp: Vec<Vec<usize>>, wp: Vec<Vec<usize>>, nu: usize) {
  // Init couples
  let mut couple: Vec<Option<usize>> = vec![None; nu];
  // Find solution
  while couple.contains(&None) {
    // Get next man
    let mi = couple.iter().position(|&e| e == None);
    if let Some(mi) = mi {
      // Check each of him preferences
      if let Some(mip) = mp.get(mi) {
        for wi in mip {
          // Check if the woman is available
          if !couple.contains(&Some(*wi)) {
            couple[mi] = Some(*wi);
            break;
          } else {
            // Get current fiance
            let c = couple.iter().position(|&e| e == Some(*wi));
            if let Some(c) = c {
              if let Some(wip) = wp.get(*wi) {
                let a = wip.iter().position(|&e| e == mi);
                let b = wip.iter().position(|&e| e == c);
                // Check if the woman prefers the new man and update marriage
                if let (Some(a), Some(b)) = (a, b) {
                  if a < b {
                    couple[mi] = Some(*wi);
                    couple[c] = None;
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
  for (i, c) in couple.iter().enumerate() {
    println!("{} {}", i + 1, c.unwrap() + 1);
  }
}

/// Run stable marriage program
fn stable_marriage() {
  // Get arguments
  let args: Vec<String> = env::args().collect();
  if args.len() > 1 {
    // Get input from a file
    let result = input(String::from(&args[1]));
    // Check if the input file was read successifully
    if let Ok(mut lines) = result {
      // Get number of test cases
      let result = get_int(&mut lines);
      if let Ok(mut ntc) = result {
        // Run test cases
        while ntc > 0 {
          // Get number of couples
          let result = get_int(&mut lines);
          if let Ok(nc) = result {
            // Read men and women preferences
            let wp = get_preferences(&mut lines, nc);
            let mp = get_preferences(&mut lines, nc);
            // Check if the preferences are not valid
            if mp.is_empty() || wp.is_empty() ||
              mp.len() != nc || wp.len() != nc {
                println!("ERROR: Invalid file format.");
                println!("  Error detail: `Invalid preferences format.`.");
                break;
            }
            // Solve stable marriage problem
            gale_shapley(mp, wp, nc);
            // Next test case
            ntc -= 1;
          } else if let Err(ref error) = result {
            println!("ERROR: Invalid file format.");
            println!("  Error detail: `{:?}`.", error);
            break;
          }
        }
      } else if let Err(ref error) = result {
        println!("ERROR: Invalid file format.");
        println!("  Error detail: `{:?}`.", error);
      }
    } else if let Err(ref error) = result {
      println!("ERROR: Sorry, you provided an invalid file.");
      println!("  Error detail: `{:?}`.", error);
    }
  } else {
    println!("WARNING: You must to enter a file with the required inputs.");
  }
}

fn main() {
  let start = Instant::now();
  stable_marriage();
  println!("Duration: {:?}.", start.elapsed());
}
