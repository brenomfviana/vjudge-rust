use std::io;

type Graph = Vec<Vec<(usize, isize)>>;

/// Reads a user input line.
fn read_line() -> String {
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Error: Unable to read user input.");
  input
}

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
  // Read the wormholes
  let wormholes: Vec<usize> = read_line().split(' ')
    .map(|s| s.trim().parse::<usize>())
    .filter_map(Result::ok).collect();
  // Check if the number of wormholes is invalid
  if wormholes.len() != 2 { panic!("Invalid number of wormholes.") }
  let (nodes, edges) = (wormholes[0], wormholes[1]);
  // Create graph
  let mut i = edges as isize - 1;
  let mut grph: Graph = vec![vec![]; nodes];
  // Read wormhole connections
  loop {
    // There is no connection
    if edges == 0 { break; }
    // Read connection
    let edge: Vec<isize> = read_line().split(' ')
      .map(|s| s.trim().parse::<isize>())
      .filter_map(Result::ok).collect();
    // Check if the connection is invalid
    if edge.len() != 3 { panic!("Invalid connection.") }
    // Get from and to wormholes and their connections
    let (f, t, w) = (edge[0] as usize, edge[1] as usize, edge[2]);
    grph[f].push((t, w));
    // Check if all connections were read
    if i <= 0 { break; } i -= 1;
  }
  grph
}

fn main() {
  // Read the number of test cases
  let ntc = read_line().trim().parse::<isize>()
    .expect("Error: The given number of test cases is invalid.");
  // Run test cases
  for _ in 0..ntc {
    // Read graph
    let grph = read_wormholes();
    // Check if there is a negative cycle and print solution
    if solve(grph) { println!("possible"); }
    else { println!("not possible"); }
  }
}
