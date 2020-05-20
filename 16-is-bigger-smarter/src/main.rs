use std::io;
use std::cmp::{Ordering, PartialEq, Eq, PartialOrd, Ord};

#[derive(Debug)]
struct Elephant {
  pub id: usize,
  pub kg: usize,
  pub iq: usize,
}

impl PartialEq for Elephant {
  fn eq(&self, other: &Self) -> bool {
    (self.id, &self.kg, &self.iq) == (other.id, &other.kg, &other.iq)
  }
}

impl PartialOrd for Elephant {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Eq for Elephant { }

impl Ord for Elephant {
  fn cmp(&self, other: &Self) -> Ordering { self.kg.cmp(&other.kg) }
}

type Bunch = Vec<Elephant>;

/// Finds the largest decreasing sequence of smallest and smartest elephants.
fn solve(elephants: &Bunch) -> Vec<usize> {
  // Cache to save the largest sequences from each elephant
  let mut cache: Vec<usize> = vec![0; elephants.len()];
  // Next elephants of the largest sequences
  let mut next: Vec<isize> = vec![-1; elephants.len()];
  // Size and the first element index of the largest sequence
  let (mut largest, mut first): (usize, isize) = (0, 0);
  // Find the largest decreasing sequence of smaller and smarter elephants
  for i in (0..(elephants.len() - 1)).rev() {
    // Each elephant is in a sequence composed by at least itself
    cache[i] = 1;
    // Look all elephants
    for j in (i + 1)..elephants.len() {
      // Which elephant is smaller?
      let j_is_smaller = elephants[j].kg > elephants[i].kg;
      // Which elephant is smarter?
      let i_is_smarter = elephants[i].iq > elephants[j].iq;
      // Add the smaller and smarter elephant
      if j_is_smaller && i_is_smarter && cache[j] + 1 > cache[i] {
        // Starting from the `i`th elephant there are `cache[j] + 1`
        // elephants in the sequence
        cache[i] = cache[j] + 1;
        // Add the elephant of the largest sequence
        next[i] = j as isize;
      }
    }
    // Update the largest sequence size and its respective first elephant
    if cache[i] > largest { largest = cache[i]; first = i as isize; }
  }
  // Build the final solution and return it
  let mut solution: Vec<usize> = vec![first as usize];
  while first != -1 && largest > solution.len() {
    first = next[first as usize] as isize;
    solution.push(first as usize);
  }
  solution
}

fn main() {
  // A vector to hold all the elephants
  let mut elephants: Bunch = vec![];
  // Index of the elephants
  let mut id = 1;
  // Read all elephants
  loop {
    // Read an elephant
    let mut input = String::new();
    io::stdin().read_line(&mut input)
      .expect("Error: Unable to read user input.");
    // Check if there is no more elephants to add
    if input == "" { break; }
    // Get the elephant data
    let elephant: Vec<usize> = input.split(" ")
      .map(|s| s.trim().parse::<usize>())
      .filter_map(Result::ok).collect();
    // Check if the read data is invalid
    if elephant.len() == 0 || elephant.len() > 2 { break; }
    // Get the elephant data
    let (kg, iq) = (elephant[0], elephant[1]);
    // Check if the read data is invalid
    if kg < 1 && iq < 1 && kg > 10000 && iq > 10000 { return (); }
    // Add elephants
    elephants.push(Elephant { id, kg, iq });
    // Update id
    id += 1;
  }
  // Decreasing sort bunch of elephants by weigth
  elephants.sort();
  // Solve problem
  let solution = solve(&elephants);
  // Print solution
  println!("{}", solution.len());
  for s in solution.iter() { println!("{}", elephants[*s].id); }
}
