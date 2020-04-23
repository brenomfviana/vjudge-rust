use std::io;
use std::collections::{HashMap, VecDeque};

/// Puzzle goal.
const GOAL:usize = 123456789;

/// The edge structs holds the state destination and the move label.
struct Edge { pub to: usize, pub label: String }

impl Edge {
  /// Returns an unitialized Edge.
  fn new() -> Edge { Edge{ to: 0, label: String::new() } }
}

/// Reads the input from the user and returns a puzzle configuration.
fn read_board() -> usize {
  // Read a full puzzle
  let mut input = String::new();
  for _ in 0..3 {
    io::stdin().read_line(&mut input)
      .expect("Error: Unable to read user input.");
  }
  let data = &input.trim().to_string();
  // Check if the puzzle is valid
  if data.len() != 17 { return 0; }
  // Initialize the current board state
  let mut board: usize = 0;
  for c in data.chars() {
    if let Some(c) = c.to_digit(10) { board = board * 10 + c as usize; }
  }
  // Return the board
  board
}

/// Builds the state graph according to the goal.
fn build_state_graph() -> HashMap<usize, Edge> {
  // State graph
  let mut state_graph: HashMap<usize, Edge> = HashMap::new();
  state_graph.insert(GOAL, Edge::new());
  // Initialize queue
  let mut deque: VecDeque<usize> = VecDeque::new();
  deque.push_back(GOAL);
  // Initialize puzzle board
  let mut board: Vec<Vec<usize>> = vec![vec![0; 3]; 3];
  // Build state graph
  while !deque.is_empty() {
    // Get next state
    if let Some(state) = deque.pop_front() {
      // Build board
      let mut aux = state;
      for i in (0..=2).rev() {
        for j in (0..=2).rev() {
          board[i][j] = aux % 10;
          aux = aux / 10;
        }
      }
      // Apply horizontal moves
      for i in 0..3 {
        // Perform moves
        let aux = board[i][0];
        for j in 0..3 {
          if j == 2 {
            board[i][j] = aux;
          } else {
            board[i][j] = board[i][j + 1];
          }
        }
        // Get the corresponding state key
        let mut from: usize = 0;
        for j in 0..3 {
          for k in 0..3 {
            from = from * 10 + board[j][k];
          }
        }
        // Build edge and add to the state graph
        if !state_graph.contains_key(&from) {
          let mut edge = Edge::new();
          edge.to = state;
          edge.label = format!("{}{}", "H", i + 1);
          state_graph.insert(from, edge);
          deque.push_back(from);
        }
        // Undo move
        let aux = board[i][2];
        for j in (0..=2).rev() {
          if j == 0 {
            board[i][j] = aux;
          } else {
            board[i][j] = board[i][j - 1];
          }
        }
      }
      // Apply vertical moves
      for j in (0..=2).rev() {
        // Perform moves
        let aux = board[2][j];
        for i in (0..=2).rev() {
          if i == 0 {
            board[i][j] = aux;
          } else {
            board[i][j] = board[i - 1][j];
          }
        }
        // Get the corresponding state key
        let mut from: usize = 0;
        for j in 0..3 {
          for k in 0..3 {
            from = from * 10 + board[j][k];
          }
        }
        // Build edge and add to the state graph
        if !state_graph.contains_key(&from) {
          let mut edge = Edge::new();
          edge.to = state;
          edge.label = format!("{}{}", "V", j + 1);
          state_graph.insert(from, edge);
          deque.push_back(from);
        }
        // Undo move
        let aux = board[0][j];
        for i in 0..3 {
          if i == 2 {
            board[i][j] = aux;
          } else {
            board[i][j] = board[i + 1][j];
          }
        }
      }
    }
  }
  state_graph
}

/// Solves the .
fn solve(state_graph: &HashMap<usize, Edge>, state: usize, mut moves: String)
  -> String {
    // Check if the goal was reached or if the state is invalid
    if state == GOAL || !state_graph.contains_key(&state) { return moves; }
    // Add move
    let edge = state_graph.get(&state).unwrap();
    moves.push_str(&edge.label);
    // Keep searching
    return solve(state_graph, edge.to, moves);
}

fn main() {
  // Build state graph
  let state_graph = build_state_graph();
  // Run test cases
  loop {
    // Read board
    let board = read_board();
    // Check if the stop condition is valid (board is equal to '0')
    if board == 0 { break; }
    // Find moves to solve the puzzle
    let moves: String = solve(&state_graph, board, String::new());
    // Print result
    if moves.is_empty() {
      println!("Not solvable");
    } else {
      println!("{} {}", moves.len() / 2, moves);
    }
  }
}
