use std::collections::{HashMap, VecDeque};
use std::io;

const UNCOLORED: isize = -1;

// Adjacency matrix
type Graph = HashMap<usize, HashMap<usize, bool>>;

/// Reads a user input line.
fn read_line() -> String {
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Error: Unable to read user input.");
  input
}

/// Return true if the graph can be bicolored or false otherwise.
fn solve(grph: Graph) -> bool {
  // Start
  let start: usize = 0;
  // BFS deque
  let mut deque: VecDeque<usize> = VecDeque::new();
  deque.push_back(start);
  // Vector to save the color of each vertex
  let mut colored: Vec<isize> = vec![UNCOLORED; grph.keys().len()];
  colored[start] = 0;
  // Run BFS
  while let Some(node) = deque.pop_front() {
    // Visite all adjacent vertex from `node`
    for i in 0..grph.len() {
      // If there is no connection between the nodes
      if !grph[&node][&i] { continue; }
      // If the node `i` is uncolored
      if colored[i] == UNCOLORED {
        // Add `i` on the deque and color it
        deque.push_back(i); colored[i] = 1 - colored[node];
      }
      // If at least one neighbor has the same color as `n`
      // the graph cannot be bicolored
      else if colored[node] == colored[i] { return false; }
    }
  }
  true
}

fn main() {
  loop {
    // Read the number of nodes
    let nodes = read_line().trim().parse::<usize>()
      .expect("Invalid number of nodes.");
    // Check if the end condition was reached
    if nodes == 0 { break; }
    // Check if the number of nodes is invalid
    if nodes >= 200 { panic!("Invalid number of nodes.") }
    // Create graph (adjacency matrix)
    let mut grph: Graph = HashMap::new();
    for i in 0..nodes {
      // Insert an empty adjacency list
      grph.insert(i, HashMap::new());
      // Get the just added adjacency list
      let adjlst = grph.get_mut(&i).expect("Invalid adjacency list.");
      // Initialize the adjacency list
      for j in 0..nodes { adjlst.insert(j, false); }
    }
    // Read the graph connections
    let edges = read_line().trim().parse::<usize>()
      .expect("Invalid number of edges.");
    for _ in 0..edges {
      // Get edge
      let values: Vec<usize> = read_line().split(' ')
        .map(|s| s.trim().parse::<usize>())
        .filter_map(Result::ok).collect();
      // Check if the read data is invalid
      if values.len() != 2 { break; }
      let (i, j) = (values[0], values[1]);
      // Check if `i` and `j` are invalid nodes
      if i >= nodes || j >= nodes { panic!("Invalid nodes.") }
      // Add edge between the nodes `i` and `j`
      let adjlst = grph.get_mut(&i).expect("Invalid adjacency list.");
      adjlst.insert(j, true);
      let adjlst = grph.get_mut(&j).expect("Invalid adjacency list.");
      adjlst.insert(i, true);
    }
    // Check if the graph can be bicolored and print solution
    if solve(grph) { println!("BICOLORABLE."); }
    else { println!("NOT BICOLORABLE."); }
  }
}
