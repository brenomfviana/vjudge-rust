use std::io;

type Graph = Vec<Vec<(usize, isize)>>;

/// Returns true if there is a negative cycle in the wormhole graph and false
/// otherwise.
fn solve(grph: Graph) -> bool {
  // Auxiliary vector
  let mut distance: Vec<i64> = vec![std::usize::MAX as i64; grph.len()];
  // Applies Bellman-Ford's Algorithm
  for _ in 0..(grph.len() - 1) {
    for i in 0..grph.len() {
			for j in 0..grph[i].len() {
				let (v, w) = grph[i][j];
				if distance[v] > (distance[i] + w as i64) {
          distance[v] = distance[i] + w as i64;
        }
			}
    }
  }
  // Check if there is a negative cycle
  for i in 0..grph.len() {
    for j in 0..grph[i].len() {
			let (v, w) = grph[i][j];
			if distance[v] > distance[i] + w as i64 { return true; }
		}
  }
	false
}

/// Reads the wormholes and returns the corresponding graph.
fn read_wormholes() -> Graph {
  // Read server config
  let mut input = String::new();
  io::stdin().read_line(&mut input)
    .expect("Error: Unable to read user input.");
  let wormholes: Vec<usize> = input.split(" ")
    .map(|s| s.trim().parse::<usize>())
    .filter_map(Result::ok).collect();
  // Check if the read data is invalid
  if wormholes.len() != 2 { panic!("Invalid file."); }
  let (nodes, edges) = (wormholes[0], wormholes[1]);
  // Create network graph
  let mut i = edges as isize - 1;
  let mut grph: Graph = vec![vec![]; nodes];
  // Read connections
  loop {
    // There is any connection
    if edges == 0 { break; }
    // Read connection
    let mut input = String::new();
    io::stdin().read_line(&mut input)
      .expect("Error: Unable to read user input.");
    let edge: Vec<isize> = input.split(" ")
      .map(|s| s.trim().parse::<isize>())
      .filter_map(Result::ok).collect();
    if edge.len() != 3 { panic!("Invalid file."); }
    // Get from and to wormholes and their connections
    let (f, t, w) = (edge[0] as usize, edge[1] as usize, edge[2]);
    grph[f].push((t, w));
    // Check if all connections were read
    if i <= 0 { break; } i -= 1;
  }
  grph
}

fn main() {
  // Read number of test cases
  let mut input = String::new();
  io::stdin().read_line(&mut input)
    .expect("Error: Unable to read user input.");
  let ntc = input.trim().parse::<isize>();
  // Check if the number of test cases was read
  if let Ok(mut ntc) = ntc {
    // Run test cases
    while ntc > 0 {
      // Read graph
      let grph = read_wormholes();
      // Check if there is a negative cycle and print solution
      if solve(grph) { println!("possible"); }
      else { println!("not possible"); }
      // Next case
      ntc -= 1;
    }
  }
}
