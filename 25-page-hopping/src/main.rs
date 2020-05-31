use std::io;

const INFINITY: isize = std::i16::MAX as isize;

type Graph = Vec<Vec<isize>>;

/// Calculates the average length between pages and return it.
fn solve(grph: &mut Graph, nodes: &Vec<usize>) -> f64 {
  // Run Floyd-Warshall's Algorithm
  for k in 0..nodes.len() {
    for i in 0..nodes.len() { if k == i { continue; }
      for j in (i + 1)..nodes.len() {
        let (x, y, z) = (nodes[i], nodes[j], nodes[k]);
        if grph[x][y] > grph[x][z] + grph[z][y] {
          grph[x][y] = grph[x][z] + grph[z][y];
        }
        if grph[y][x] > grph[y][z] + grph[z][x] {
          grph[y][x] = grph[y][z] + grph[z][x];
        }
      }
    }
  }
  // Calculate the average shortest path length between every pair of nodes
  let mut cost: f64 = 0.0;
  for i in 0..nodes.len() {
    for j in (i + 1)..nodes.len() {
      let (x, y) = (nodes[i], nodes[j]);
      if grph[x][y] != INFINITY { cost += grph[x][y] as f64; }
      if grph[y][x] != INFINITY { cost += grph[y][x] as f64; }
    }
  }
  cost /= nodes.len() as f64 * (nodes.len() - 1) as f64;
  // Return the average shortest path length between every pair of nodes
  cost
}

/// Reads and returns a tuple of usize.
fn read_graph() -> (Graph, Vec<usize>) {
  // Read graph
  let mut input = String::new();
  io::stdin().read_line(&mut input)
    .expect("Error: Unable to read user input.");
  let edges: Vec<usize> = input.split(" ")
    .map(|s| s.trim().parse::<usize>())
    .filter_map(Result::ok).collect();
  // Check if the read data is invalid
  if edges.len() % 2 != 0 { panic!("Invalid file."); }
  // Get build the graph according with the max element
  if let Some(&max) = edges.iter().max() {
    // Build graph
    let mut grph: Graph = vec![vec![INFINITY; max]; max];
    let mut nodes = vec![];
    // Add elements
    let mut k = 0;
    loop {
      // Check if the end condition was reached
      if edges[k] == 0 && edges[k + 1] == 0 { break; }
      // Add an edge
      let (i, j) = (edges[k] - 1, edges[k + 1] - 1);
      grph[i][j] = 1;
      // Add indexes
      if !nodes.contains(&i) { nodes.push(i); }
      if !nodes.contains(&j) { nodes.push(j); }
      // Next edge
      k += 2;
    }
    // Return graph
    nodes.sort();
    return (grph, nodes);
  }
  (vec![], vec![])
}

fn main() {
  // Test cases iterator
  let mut itc = 1;
  loop {
    // Read map and find the average length between pages
    let (mut grph, nodes) = read_graph();
    // Check if graph is empty
    if grph.is_empty() { break; }
    // Calculate the average shortest path length between every pair of nodes
    let cost = solve(&mut grph, &nodes);
    // Print solution
    print!("Case {}: average length between pages", itc);
    println!(" = {:.3} clicks", cost);
    // Next test case
    itc += 1;
  }
}
