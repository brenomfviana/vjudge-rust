use std::io;

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
      if arr.len() > 0 && larr2 > 1 { inv += 1; }
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
  // Read number of test cases
  let mut input = String::new();
  io::stdin().read_line(&mut input)
    .expect("Error: Unable to read user input.");
  let ntc = input.trim().parse::<usize>();
  // Check if the number of test cases was read
  if let Ok(mut ntc) = ntc {
    // Run test cases
    while ntc > 0 {
      // Read array size
      let mut input = String::new();
      io::stdin().read_line(&mut input)
        .expect("Error: Unable to read user input.");
      let _ = input.trim().parse::<usize>().unwrap();
      // Read array
      let mut input = String::new();
      io::stdin().read_line(&mut input)
        .expect("Error: Unable to read user input.");
      let array: Vec<usize> = input.split(" ")
        .map(|s| s.trim().parse::<usize>())
        .filter_map(Result::ok).collect();
      // Find the number of inversions
      let (inv, _) = solve(array);
      println!("{}", inv);
      // Next test case
      ntc -= 1;
    }
  }
}
