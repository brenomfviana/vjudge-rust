use std::io;

/// Applies Gale-Shapley's algorithm
fn gale_shapley(mp: Vec<Vec<usize>>, wp: Vec<Vec<usize>>, nu: usize) {
  // Init couples
  let mut couples: Vec<Option<usize>> = vec![None; nu];
  // Find solution
  while couples.contains(&None) {
    // Get next man
    let mi;
    if let Some(i) = couples.iter().position(|&e| e == None) { mi = i; }
    else { panic!("Invalid man index."); }
    // Get man preferences
    let mip;
    if let Some(p) = mp.get(mi) { mip = p; }
    else { panic!("Invalid man preferences."); }
    // Check each of him preferences
    for wi in mip {
      // Check if the woman is available
      if !couples.contains(&Some(*wi)) { couples[mi] = Some(*wi); break; }
      else {
        // Get current fiance
        let f;
        if let Some(c) = couples.iter().position(|&e| e == Some(*wi)) { f = c; }
        else { panic!("Invalid man index."); }
        // Get woman preferences
        let wip;
        if let Some(p) = wp.get(*wi) { wip = p; }
        else { panic!("Invalid woman preferences."); }
        // Compare the woman current fiance and the new proposer
        let proposer = wip.iter().position(|&e| e == mi);
        let fiance = wip.iter().position(|&e| e == f);
        // Check if the woman prefers the new man and update marriage
        if let (Some(proposer), Some(fiance)) = (proposer, fiance) {
          if proposer < fiance {
            couples[mi] = Some(*wi);
            couples[f] = None;
            break;
          }
        } else { panic!("The .") }
      }
    }
  }
  // Print solution
  for (i, couple) in couples.iter().enumerate() {
    let c;
    if let Some(a) = couple { c = a; }
    else { panic!("There is an invalid couple."); }
    println!("{} {}", i + 1, c + 1);
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
    preferences.push(input.split(' ')
      .enumerate()
      .filter(|(i, _)| *i > 0)
      .map(|(_, s)| s.trim().parse::<usize>())
      .filter_map(Result::ok)
      .map(|s| s - 1)
      .collect::<Vec<usize>>());
  }
  preferences
}

fn main() {
  // Get number of test cases
  let mut input = String::new();
  io::stdin().read_line(&mut input)
    .expect("Error: Unable to read user input.");
  // Check if the number of test cases was read
  if let Ok(mut ntc) = input.trim().parse::<usize>() {
    // Run test cases
    while ntc > 0 {
      // Get number of couples
      let mut input = String::new();
      io::stdin().read_line(&mut input)
        .expect("Error: Unable to read user input.");
      // Check if the number of couples was read
      if let Ok(nc) = input.trim().parse::<usize>() {
        // Read men and women preferences
        let wp = get_preferences(nc);
        let mp = get_preferences(nc);
        // Check if the preferences are not valid
        if mp.len() != nc || wp.len() != nc { break; }
        // Solve stable marriage problem
        gale_shapley(mp, wp, nc);
        // Next test case
        ntc -= 1;
      } else { break; }
    }
  }
}
