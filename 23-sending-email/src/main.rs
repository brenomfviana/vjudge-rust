use std::io;
use std::collections::VecDeque;

type Graph = Vec<Vec<(usize, usize)>>;

/// Reads a user input line.
fn read_line() -> String {
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Error: Unable to read user input.");
  input
}

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
  latency[t]
}

/// Reads the server config and returns the graph and the from and to servers.
#[allow(clippy::many_single_char_names)]
fn read_server_config() -> (Graph, usize, usize) {
  // Read server config
  let mut input = read_line();
  // Ignore empty lines
  while input.trim() == "" { input = read_line(); }
  // Convert line to server config
  let server: Vec<usize> = input.split(' ')
    .map(|s| s.trim().parse::<usize>())
    .filter_map(Result::ok).collect();
  // Check if the read data is invalid
  if server.len() != 4 { panic!("Invalid server.") }
  // Assign the server config to the respective variables
  let (n, m, s, t) = (server[0], server[1], server[2], server[3]);
  // Check if the server config is invalid
  if n > 20000 || m > 50000 || s > n || t > n { panic!("Invalid server.") }
  // Create network graph
  let mut i = m as isize - 1;
  let mut grph: Graph = vec![vec![]; n];
  // Read connections
  loop {
    // There is any connection
    if m == 0 { break; }
    // Read connection
    let input = read_line();
    // Ignore empty lines
    if input.trim() == "" { continue; }
    let conn: Vec<usize> = input.split(' ')
      .map(|s| s.trim().parse::<usize>())
      .filter_map(Result::ok).collect();
    // Check if the connection is invalid
    if conn.len() != 3 { panic!("Invalid file.") }
    // Get from and to servers and their connection latency
    let (f, t, l) = (conn[0], conn[1], conn[2]);
    // Add connection
    grph[f].push((t, l)); grph[t].push((f, l));
    // Check if all connections were read
    if i <= 0 { break; } i -= 1;
  }
  (grph, s, t)
}

fn main() {
  // Read the number of test cases
  let ntc = read_line().trim().parse::<isize>()
    .expect("Error: The given number of test cases is invalid.");
  // Run test cases
  for (itc, _) in (0..ntc).enumerate() {
    // Read graph
    let (grph, s, t) = read_server_config();
    // Applies Dijkstra's Algorithm
    let dist = solve(grph, s, t);
    // Print solution
    print!("Case {}: ", itc + 1);
    if dist != std::usize::MAX { println!("{:?}", dist); }
    else { println!("unreachable"); }
  }
}
