use std::io;
use std::collections::{HashMap, VecDeque};

const UNCOLORED: isize = -1;

// Adjacency matrix
type Graph = HashMap<usize, HashMap<usize, bool>>;

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
    // Read the input
    let mut input = String::new();
    io::stdin().read_line(&mut input)
      .expect("Error: Unable to read user input.");
    // Check if the number of nodes was successfully read
    if let Ok(nodes) = input.trim().parse::<usize>() {
      // Check if the end condition was reached
      if nodes == 0 { break; }
      // Create graph (adjacency matrix)
      let mut grph: Graph = HashMap::new();
      for i in 0..nodes {
        // Insert an empty adjacency list
        grph.insert(i, HashMap::new());
        // Get the just added adjacency list
        if let Some(adjlst) = grph.get_mut(&i) {
          // Initialize the adjacency list
          for j in 0..nodes { adjlst.insert(j, false); }
        }
      }
      // Read graph connections
      // Read the input
      let mut input = String::new();
      io::stdin().read_line(&mut input)
        .expect("Error: Unable to read user input.");
      // Check if the number of nodes was successfully read
      if let Ok(edges) = input.trim().parse::<usize>() {
        for _ in 0..edges {
          // Read the input
          let mut input = String::new();
          io::stdin().read_line(&mut input)
            .expect("Error: Unable to read user input.");
          // Get values
          let values: Vec<usize> = input.split(" ")
            .map(|s| s.trim().parse::<usize>())
            .filter_map(Result::ok).collect();
          // Check if the read data is invalid
          if values.len() != 2 { break; }
          let (i, j) = (values[0], values[1]);
          // Add edge between the nodes `i` and `j`
          if let Some(adjlst) = grph.get_mut(&i) { adjlst.insert(j, true); }
          if let Some(adjlst) = grph.get_mut(&j) { adjlst.insert(i, true); }
        }
        // Check if the graph can be bicolored and print solution
        if solve(grph) { println!("BICOLORABLE."); }
        else { println!("NOT BICOLORABLE."); }
      } else { panic!("Invalid input!"); }
    } else { panic!("Invalid input!"); }
  }
}
