use std::io;

/// Reads a user input line.
fn read_line() -> String {
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Error: Unable to read user input.");
  input
}

/// Applies Gale-Shapley's algorithm
fn gale_shapley(mp: Vec<Vec<usize>>, wp: Vec<Vec<usize>>, nu: usize) {
  // Init couples
  let mut cpls: Vec<Option<usize>> = vec![None; nu];
  // Find solution
  while cpls.contains(&None) {
    // Get next man
    let mi= cpls.iter().position(|&e| e == None).expect("Invalid man index.");
    // Get man preferences
    let mip = mp.get(mi).expect("Invalid man preferences.");
    // Check each of him preferences
    for wi in mip {
      // Check if the woman is available
      if !cpls.contains(&Some(*wi)) { cpls[mi] = Some(*wi); break; }
      else {
        // Get woman preferences
        let wip = wp.get(*wi).expect("Invalid woman preferences.");
        // Get current fiancé
        let sc = |&e| e == Some(*wi);
        let f = cpls.iter().position(sc).expect("Invalid fiancé index.");
        let sc = |&e| e == f;
        let fiance = wip.iter().position(sc).expect("Invalid fiancé.");
        // Get woman current proposer
        let sc = |&e| e == mi;
        let proposer = wip.iter().position(sc).expect("Invalid proposer.");
        // Check if the woman prefers the new man and update marriage
        if proposer < fiance { cpls[mi] = Some(*wi); cpls[f] = None; break; }
      }
    }
  }
  // Print solution
  for (i, cpl) in cpls.iter().enumerate() {
    let c = cpl.expect("Invalid couple.");
    println!("{} {}", i + 1, c + 1);
  }
}

/// Gets the person's preferences
fn get_preferences(size: usize) -> Vec<Vec<usize>> {
  // Init preferences list
  let mut preferences: Vec<Vec<usize>> = vec![];
  // For each partner
  for _ in 0..size {
    // Split the preferences line
    preferences.push(read_line().split(' ').enumerate()
      .filter(|(i, _)| *i > 0).map(|(_, s)| s.trim().parse::<usize>())
      .filter_map(Result::ok).map(|s| s - 1).collect::<Vec<usize>>());
  }
  preferences
}

fn main() {
  // Read the number of test cases
  let ntc = read_line().trim().parse::<usize>()
    .expect("Error: The given number of test cases is invalid.");
  // Check if the number of test cases is invalid
  if ntc > 100 { panic!("Invalid number of test cases.") }
  // Run test cases
  for _ in 0..ntc {
    // Read the number of couples
    let nc = read_line().trim().parse::<usize>()
      .expect("Error: The given number of couples is invalid.");
    // Check if the number of couples is invalid
    if nc > 500 { panic!("Invalid number of couples.") }
    // Read men and women preferences
    let (wp, mp) = (get_preferences(nc), get_preferences(nc));
    // Check if the preferences are invalid
    if mp.len() != nc || wp.len() != nc { panic!("Invalid preferences.") }
    // Solve stable marriage problem
    gale_shapley(mp, wp, nc);
  }
}
