use std::io;
use std::collections::VecDeque;

type Graph = Vec<Vec<(usize, usize)>>;

/// Calculates the latency between two servers.
fn solve(grph: Graph, s: usize, t: usize) -> usize {
  // Initialize latency vector with infinity size
  let mut latency: Vec<usize> = vec![std::usize::MAX; grph.len()];
  latency[s] = 0;
  // Initialize parent list
  let mut parent: Vec<usize> = vec![0; grph.len()];
  // Initialize queue
  let mut deque: VecDeque<(usize, usize)> = VecDeque::new();
  deque.push_back((latency[s], s));
  // Repeat until the queue is empty
  while let Some((d, u)) = deque.pop_front() {
    // Consume element already visited elements
		if d > latency[u] { continue; }
		// Visit all adjacent nodes
		for (v, l) in grph[u].iter() {
			// Check if we need relax the edge
			if latency[*v] > latency[u] + l {
				latency[*v] = latency[u] + l;
				parent[*v] = u;
				deque.push_back((latency[*v], *v));
			}
		}
  }
  return latency[t];
}

/// Reads the server config and returns the graph and the from and to servers.
fn read_server_config() -> (Graph, usize, usize) {
  // Read server config
  let mut input = String::new();
  io::stdin().read_line(&mut input)
    .expect("Error: Unable to read user input.");
  // Ignore empty lines
  while input.trim() == "" {
    input = String::new();
    io::stdin().read_line(&mut input)
      .expect("Error: Unable to read user input.");
  }
  let server: Vec<usize> = input.split(" ")
    .map(|s| s.trim().parse::<usize>())
    .filter_map(Result::ok).collect();
  // Check if the read data is invalid
  if server.len() != 4 { panic!("Invalid file."); }
  let (m, n, s, t) = (server[0], server[1], server[2], server[3]);
  // Create network graph
  let mut i = n as isize - 1;
  let mut grph: Graph = vec![vec![]; m];
  // Read connections
  loop {
    // There is any connection
    if n == 0 { break; }
    // Read connection
    let mut input = String::new();
    io::stdin().read_line(&mut input)
      .expect("Error: Unable to read user input.");
    // Ignore empty lines
    if input == "" { continue; }
    let conn: Vec<usize> = input.split(" ")
      .map(|s| s.trim().parse::<usize>())
      .filter_map(Result::ok).collect();
    if conn.len() != 3 { panic!("Invalid file."); }
    // Get from and to servers and the connection latency
    let (f, t, l) = (conn[0], conn[1], conn[2]);
    grph[f].push((t, l)); grph[t].push((f, l));
    // Check if all connections were read
    if i <= 0 { break; } i -= 1;
  }
  (grph, s, t)
}

fn main() {
  // Read number of test cases
  let mut input = String::new();
  io::stdin().read_line(&mut input)
    .expect("Error: Unable to read user input.");
  let ntc = input.trim().parse::<isize>();
  // Check if the number of test cases was read
  if let Ok(mut ntc) = ntc {
    // Test case iterator
    let mut itc = 0;
    // Run test cases
    while ntc > 0 {
      // Read graph
      let (grph, s, t) = read_server_config();
      // Applies Dijkstra's Algorithm
      let dist = solve(grph, s, t);
      // Print solution
      print!("Case {}: ", itc + 1);
      if dist != std::usize::MAX { println!("{:?}", dist); }
      else { println!("unreachable"); }
      // Next case
      ntc -= 1; itc += 1;
    }
  }
}
