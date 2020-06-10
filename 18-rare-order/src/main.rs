use std::io;
use std::cmp;
use std::collections::{HashMap, VecDeque};

// Lisf of the adjacency vectors
type AdjLst = Vec<char>;
// Adjacency Matrix
type AdjMtx = HashMap<char, AdjLst>;
// In-degree of vertices
type InDegree = HashMap<char, usize>;

/// Reads a user input line.
fn read_line() -> String {
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Error: Unable to read user input.");
  input
}

/// Apply topological sorting (based on Kahn’s algorithm) on the vocabulary
/// (graph) and return a string with the sorted nodes.
fn solve(grph: AdjMtx) -> String {
  // Create a vector to store in-degrees of all vertices
  let mut indegree: InDegree = HashMap::new();
  for node in grph.keys() { indegree.insert(*node, 0); }
  // Traverse adjacency lists to fill in-degrees of vertices | complexity=O(V+E)
  for node in grph.keys() {
    for nghbr in &grph[&node] { indegree.insert(*nghbr, indegree[&nghbr] + 1); }
  }
  // Initialize deque and enqueue all vertices with in-degree 0
  let mut deque: VecDeque<char> = VecDeque::new();
  for (node, indgr) in &indegree { if *indgr == 0 { deque.push_back(*node); } }
  // Initialize count of visited vertices
  let mut cnt: usize = 0;
  // Order of the words in the strange alphabet
  let mut order = String::new();
  // Run BFS
  while let Some(node) = deque.pop_front() {
    // Add the node to topological order
    order.push(node);
    // Iterate through all its neighbouring nodes of dequeued node u and
    for nghbr in &grph[&node] {
      // Decrease the neighbor in-degree by 1
      indegree.insert(*nghbr, indegree[&nghbr] - 1);
      // If in-degree becomes zero, add it to queue
      if indegree[nghbr] == 0 { deque.push_back(*nghbr); }
    }
    cnt += 1;
  }
  // Check if there was a cycle
  if cnt != grph.keys().len() {
    println!("There exists a cycle in the graph\n");
    return String::from("")
  }
  // Return final result
  order
}

fn main() {
  loop {
    // Auxiliary variables
    let (mut pln, mut ln): (Vec<char>, Vec<char>); ln = vec![];
    // Create adjacency matrix
    let mut grph: AdjMtx = HashMap::new();
    // Read the input
    let input = read_line();
    // Check if the end condition was reached
    if input == "" { break; }
    // Read the first line
    pln = input.split("").map(|s| s.trim().parse::<char>())
      .filter_map(Result::ok).collect();
    // Read all words
    while ln != vec!['#'] {
      // Check if the string is invalid
      if ln.len() > 20 { panic!("Invalid string.") }
      // Check if the next line was read
      if ln != vec![] {
        // Compare current line with the previous one
        for i in 0..(cmp::min(ln.len(), pln.len())) {
          // If the `i`-th char of both lines are different then relate them
          if ln[i] != pln[i] {
            // Connect nodes
            if let Some(adjlst) = grph.get_mut(&pln[i]) { adjlst.push(ln[i]); }
            else { grph.insert(pln[i], vec![ln[i]]); }
            // If the node was not added yet then add it
            if grph.get_mut(&ln[i]).is_none() { grph.insert(ln[i], vec![]); }
            break;
          }
        }
        // Update previous line
        pln = ln;
      }
      // Read the next line
      ln = read_line().split("").map(|s| s.trim().parse::<char>())
        .filter_map(Result::ok).collect();
    }
    // Run topological sort and print result
    println!("{}", solve(grph));
  }
}
