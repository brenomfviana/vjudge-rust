use std::io;
use std::cmp;
use std::collections::VecDeque;

type Graph = Vec<Vec<usize>>;
type RawGraph = [Vec<usize>];
type Pair = (usize, usize);

/// Reads a user input line.
fn read_line() -> String {
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Error: Unable to read user input.");
  input
}

/// Runs the Breadth-First Search and returns true if the node `t` was reached
/// or false otherwise.
fn bfs(rgrph: &RawGraph, parent: &mut Vec<isize>, (s, t): Pair) -> bool {
	// Create visited array
	let mut visited: Vec<bool> = vec![false; rgrph.len()];
	// BFS Deque
  let mut deque: VecDeque<usize> = VecDeque::new(); deque.push_back(s);
	// Run BFS
  while let Some(u) = deque.pop_front() {
		// Visit `u` neighbors
    for v in 0..rgrph.len() {
			// Check if the neighbor was not visited yet
			if !visited[v] && rgrph[u][v] > 0 {
				parent[v] = u as isize;
				deque.push_back(v);
				visited[v] = true;
			}
    }
  }
  visited[t]
}

/// Runs Ford-Fulkerson's algorithm and returns the residual graph.
fn ff(grph: &mut Graph, parent: &mut Vec<isize>, (s, t): Pair) -> Graph {
	// Clone the graph
	let mut rgrph = grph.clone();
	// While there is a path between `s` and `t`
	while bfs(&rgrph, parent, (s, t)) {
		// Set min flow
		let mut residual = std::isize::MAX as usize;
		// Calculate minimun cost
    let mut v = t;
		loop {
      if v == s { break }
      let u = parent[v] as usize;
	  	residual = cmp::min(residual, rgrph[u][v]);
      v = u;
	  }
		// Calculate residual graph
    let mut v = t;
		loop {
      if v == s { break }
      let u = parent[v] as usize;
	  	rgrph[v][u] += residual;
      rgrph[u][v] -= residual;
      v = u;
	  }
	}
	// Return residual graph
  rgrph
}

/// Runs the Deepth-First Search to find the nodes reached from `s`;
fn dfs(rgrph: &RawGraph, s: usize, visited: &mut Vec<bool>) {
  visited[s] = true;
  for i in 0..rgrph.len() {
		if rgrph[s][i] != 0 && !visited[i] { dfs(rgrph, i, visited); }
	}
}

/// Finds the cables that need to be cutted between the cities `s` and `t`.
fn solve(mut grph: Graph) {
	// Set capital and largest city
  let (s, t): (usize, usize) = (0, 1);
  // Parent vector
  let mut parent: Vec<isize> = vec![-1; grph.len()];
	// Run Ford-Fulkerson's algorithm
  let rgrph = ff(&mut grph, &mut parent, (s, t));
	// Create visited array
	let mut visited = vec![false; rgrph.len()];
  // Run DFS to find the nodes reached from `s`
	dfs(&rgrph, s, &mut visited);
	// Print solution
	for i in 0..rgrph.len() {
		if !visited[i] { continue; }
	  for (j, _) in visited.iter().enumerate().take(rgrph.len()) {
			if !visited[j] && grph[i][j] != 0 { println!("{} {}", i + 1, j + 1); }
	  }
	}
}

fn main() {
  loop {
    // Read input
    let input = read_line();
    // Ignore empty lines
    if input.trim() == "" { continue; }
    // Get the number of nodes and edges of the graph
    let params: Vec<usize> = input.split(' ')
      .map(|s| s.trim().parse::<usize>())
      .filter_map(Result::ok).collect();
    // Check if the number of nodes and edges are invalid
    if params.len() != 2 { panic!("Invalid number of nodes and edges.") }
    // Assign the number of nodes and edges of the graph
    let (n, m) = (params[0], params[1]);
    // Check if the end condition was read
    if n == 0 && m == 0 { break; }
    // Create graph
    let mut grph = vec![vec![0; n]; n];
    // Build graph
    for _ in 0..m {
      // Read input
      let input = read_line();
      // Ignore empty lines
      if input.trim() == "" { continue; }
      // Get the number of nodes and edges of the graph
      let params: Vec<usize> = input.split(' ')
        .map(|s| s.trim().parse::<usize>())
        .filter_map(Result::ok).collect();
      // Check if the edge configuration is invalid
      if params.len() != 3 { panic!("Invalid edge.") }
      // Read connection config
      let (a, b, weight) = (params[0], params[1], params[2]);
      // Add edge
      grph[a - 1][b - 1] += weight;
      grph[b - 1][a - 1] += weight;
    }
    // Run solver
    solve(grph);
    println!();
  }
}
