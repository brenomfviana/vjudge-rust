use std::io;
use std::cmp::Reverse;

type Graph = Vec<(usize, (usize, usize))>;

/// Union find.
struct UnionFind {
  pntr: Vec<usize>,
  rank: Vec<usize>,
}

impl UnionFind {
  /// Creates a new union find.
  fn new(size: usize) -> UnionFind {
    // Initialize the rank vector
    let rank = vec![0; size];
    // Initialize the pointer vector
    let mut pntr = vec![0; size];
    // Every element start point to itself (each one of them has its own set)
    for i in 0..size { pntr[i] = i; }
    // Return new union find
    UnionFind { pntr, rank }
  }

  /// Finds the highest parent of the set and return its index.
  fn find_set(&self, i: usize) -> usize {
    // If the element points to itself, return it
    if self.pntr[i] == i { return i; }
    // Keep searching the element last pointer of `i`
    return self.find_set(self.pntr[i]);
  }

  /// Returns true if two elements are in the same set or false otherwise.
  fn is_same_set(&self, i: usize, j: usize) -> bool {
    return self.find_set(i) == self.find_set(j);
  }

  /// Unifies sets that owns the elements `i` and `j`.
  fn union_set(&mut self, i: usize, j: usize) {
    // Check if both elements already are in the same set
    if !self.is_same_set(i, j) {
      // Get `i` and `j` highest parents
      let (x, y) = (self.find_set(i), self.find_set(j));
      // If `x` has a greater rank than `y` then `y` points to `x`
      if self.rank[x] > self.rank[y] { self.pntr[y] = x; }
      else {
        // `x` now points to `y`
        self.pntr[x] = y;
        // If both are in the same rank then upgrade `y` rank
        if self.rank[x] == self.rank[y] { self.rank[y] += 1; }
      }
    }
  }
}

/// Calculates the cost of the cameras by using Kruskal's algorithm.
fn solve(mut grph: Graph, mut union_find: UnionFind) -> usize {
  // Descresing sort graph by weight
  grph.sort_by_key(|&num| Reverse(num));
  // Calculate the cost of the cameras
  let mut cost: usize = 0;
  for i in 0..grph.len() {
    // Get the `i` edge
    let (weight, (v, u)) = grph[i];
    // If `v` and `u` are not in the same set and join both sets
    if !union_find.is_same_set(v, u) { union_find.union_set(v, u); }
    // A new circle was formed so we need a camera that costs `weight`
    else { cost += weight; }
  }
  // Return the calculated cost
  cost
}

/// Reads the input graph and returns the read graph and an union find.
fn read_graph() -> (Graph, UnionFind) {
  // Read graph dimension
  let mut input = String::new();
  io::stdin().read_line(&mut input)
    .expect("Error: Unable to read user input.");
  let dimension: Vec<usize> = input.split(" ")
    .map(|s| s.trim().parse::<usize>())
    .filter_map(Result::ok).collect();
  // Check if the read data is invalid
  if dimension.len() != 2 { panic!("Invalid file."); }
  let (nodes, edges) = (dimension[0], dimension[1]);
  // Read the graph edges
  let mut grph = Graph::with_capacity(edges);
  loop {
    // Read graph edge
    let mut input = String::new();
    io::stdin().read_line(&mut input)
      .expect("Error: Unable to read user input.");
    let edge: Vec<usize> = input.split(" ")
      .map(|s| s.trim().parse::<usize>())
      .filter_map(Result::ok).collect();
    // Check if the read data is invalid
    if edge.len() != 3 { panic!("Invalid file."); }
    grph.push((edge[2], (edge[0] - 1, edge[1] - 1)));
    // Stop reading if all edges were already read
    if grph.len() == edges { break; }
  }
  // Return graph and union find
  (grph, UnionFind::new(nodes))
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
      let (grph, union_find) = read_graph();
      // Calculate the cost of the cameras and print the result
      println!("{}", solve(grph, union_find));
      // Next test case
      ntc -= 1;
    }
  }
}
