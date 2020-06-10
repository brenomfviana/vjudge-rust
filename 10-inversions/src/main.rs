use std::io;

/// Reads a user input line.
fn read_line() -> String {
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Error: Unable to read user input.");
  input
}

/// Concats two arrays, counts the number of inversions and returns the number
/// of inversions and the concated array.
fn concat(arr1: Vec<usize>, arr2: Vec<usize>) -> (usize, Vec<usize>) {
  // Get lengths
  let (larr1, larr2) = (arr1.len(), arr2.len());
  // Control variables
  let (maxlen, mut i, mut j) = (larr1 + larr2, 0, 0);
  // Inversions counter and resulting array
  let (mut inv, mut arr): (usize, Vec<usize>) = (0, vec![]);
  // Concat arrays
  while maxlen > arr.len() {
    if larr1 > i && larr2 > j && arr1[i] <= arr2[j] {
      arr.push(arr1[i]);
      i += 1;
    } else if larr1 > i && larr2 > j && arr1[i] > arr2[j] {
      arr.push(arr2[j]);
      j += 1;
      inv += 1;
    } else if larr1 > i && larr2 <= j {
      arr.push(arr1[i]);
      i += 1;
      if !arr.is_empty() && larr2 > 1 { inv += 1; }
    } else if larr1 <= i && larr2 > j {
      arr.push(arr2[j]);
      j += 1;
    }
  }
  (inv, arr)
}

/// Finds the number of inversions of an array.
fn solve(array: Vec<usize>) -> (usize, Vec<usize>) {
  // Get array size
  let size = array.len();
  // Check if the array has only one element
  if size == 1 { return (0, array); }
  // Calculate the mid
  let mid = size / 2;
  // Keep searching
  let (arr1, arr2) = array.split_at(mid);
  let (i1, arr1) = solve(arr1.to_vec());
  let (i2, arr2) = solve(arr2.to_vec());
  // Concat arrays
  let (inv, arr) = concat(arr1, arr2);
  (inv + i1 + i2, arr)
}

fn main() {
  // Read the number of test cases
  let ntc = read_line().trim().parse::<usize>()
    .expect("Error: The given number of test cases is invalid.");
  // Run test cases
  for _ in 0..ntc {
    // Read array size
    let asize = read_line().trim().parse::<usize>()
      .expect("Error: The given array size is invalid.");
    // Check if the array size is invalid
    if asize > 10000 { panic!("Invalid array size.") }
    // Read array
    let array: Vec<usize> = read_line().split(' ')
      .map(|s| s.trim().parse::<usize>())
      .filter_map(Result::ok).collect();
    // Check if the element with the highest value is valid
    if array.iter().max() >= Some(&30000) { panic!("Invalid array.") }
    // Check if the array size matches to the read array size
    assert_eq!(asize, array.len());
    // Find the number of inversions
    let (inv, _) = solve(array);
    println!("{}", inv);
  }
}
