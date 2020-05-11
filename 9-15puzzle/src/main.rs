use std::io;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

/// Defines alias.
type Position = (usize, usize);
type Board = Vec<Vec<usize>>;
type GoalMap = HashMap<usize, Position>;
type Cost = Option<usize>; // None represents positive infinity

/// Board dimension.
const D: usize = 4;
/// Posible move directions.
const DIRS: &'static [(i32, i32)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];
/// Board goal.
const GOAL: &'static [&'static [usize]] =
  &[&[1, 2, 3, 4], &[5, 6, 7, 8], &[9, 10, 11, 12], &[13, 14, 15, 0]
];

/// Defines a 15 puzzle state.
#[derive(Debug, Clone, Eq)]
struct State {
  pub board: Board,
  pub moves: String,
  pub blank: Position,
  pub depth: usize,
  pub cost: Cost,
}

impl State {
  /// Creates a new 15 puzzle state.
  fn new(board: Board, moves: String, blank: Position, depth: usize,
    cost: Cost) -> State {
      State{ board, moves, blank, depth, cost }
  }

  /// Creates an invalid new 15 puzzle state that holds only its cost.
  fn invnew(cost: Cost) -> State {
    State::new(vec![], String::new(), (D + 1, D + 1), 0, cost)
  }
}

impl PartialEq for State {
  fn eq(&self, other: &Self) -> bool { self.board == other.board }
}

impl Hash for State {
  fn hash<H: Hasher>(&self, state: &mut H) { self.board.hash(state); }
}

/// Defines the position of each element in the board goal.
fn goal_map() -> GoalMap {
  // Initialize goal map
  let mut goal_map: GoalMap = HashMap::new();
  // Add the element positions mapped by the elements
  for i in 0..D { for j in 0..D { goal_map.insert(GOAL[i][j], (i, j)); } }
  // Return goal map
  goal_map
}

/// Reads the input from the user and returns a puzzle configuration.
fn read_board() -> Board {
  // Initialize an empty board
  let mut board: Board = vec![vec![0; D]; D];
  // Read a full puzzle
  for i in 0..D {
    // Read a puzzle line
    let mut input = String::new();
    io::stdin().read_line(&mut input)
      .expect("Error: Unable to read user input.");
    let row: Vec<usize> = input.split(" ")
      .map(|s| s.trim().to_string().parse::<usize>())
      .filter_map(Result::ok).collect();
    board[i] = row;
  }
  // Return the board
  board
}

/// Finds position of blank from bottom.
fn find_blank_position(board: &Board) -> Position {
  for i in 0..=3 { for j in 0..=3 {
    if board[i][j] == 0 { return (i, j); }
  } }
  (D + 1, D + 1)
}

/// Checks if the given boad is solvable.
fn is_solvable(board: &Board) -> bool {
  // Count the number of inversions
  let mut count = 0;
  // Row where the black block is
  let mut brow = -1;
  for i in 0..(D * D - 1) {
    // Get points
    let (xi, yi) = (i / 4, i % 4);
    // Ignore empty block
    if board[xi][yi] == 0 { brow = 4 - xi as i32; continue; }
    // Count the inverted elements
    for j in (i + 1)..(D * D) {
      // Get points
      let (xj, yj) = (j / 4, j % 4);
      // Ignore empty block
      if board[xj][yj] == 0 { continue; }
      // Check if the element is inverted
      if board[xi][yi] > board[xj][yj] { count += 1; }
    }
  }
  // If the blank block is in a odd row, the inverted blocks must be even
  if brow % 2 == 0 { return count % 2 != 0; }
  // If the blank block is in a even row, the inverted blocks must be odd
  count % 2 == 0
}

// Calculates the heuristic cost.
fn h(board: &Board, goal: &GoalMap) -> usize {
  // The minimun cost is the search depth
  let mut cost = 0;
  // Calculates the manhattan distance from the element's current position
  // with its correct position
  for i in 0..D { for j in 0..D {
      // Ignore empty block
      if board[i][j] == 0 { continue; }
      // Get right number position
      let (gi, gj) = goal[&board[i][j]];
      cost += ((gi as i32 - i as i32) + (gj as i32 - j as i32)).abs();
    }
  }
  // Return heuristic cost
  cost as usize
}

/// Search for a solution by using DLS algorithm.
fn search(bound: usize, current: State, goal: &GoalMap,
  visited: &mut HashMap<State, String>) -> State {
    // Check if the board cost is higher than the bound
    let cost = current.depth + h(&current.board, goal);
    if cost > bound { return State::invnew(Some(cost)); }
    // Check if the goal was reached
    if current.board == GOAL { return current; }
    // Add to the visited states list
    visited.insert(current.clone(), current.moves.clone());
    // Defines the minimun as infinity (represented by None)
    let mut min: Cost = None;
    // Apply moves
    for direction in DIRS {
      // Check bounds and apply a new move
      let (i, j) = direction;
      let mut nmoves = current.moves.clone();
      if *i == -1 && *j == 0 { nmoves.push('U'); }
      if *i == 1 && *j == 0 { nmoves.push('D'); }
      if *i == 0 && *j == -1 { nmoves.push('L'); }
      if *i == 0 && *j == 1 { nmoves.push('R'); }
      // Calculate the new position of the blank block
      let (x, y) = current.blank;
      let (nx, ny): (Option<usize>, Option<usize>);
      if *i < 0 {
        nx = usize::checked_sub(x, i.abs() as usize);
        ny = usize::checked_add(y, *j as usize);
      }
      else if *j < 0 {
        nx = usize::checked_add(x, *i as usize);
        ny = usize::checked_sub(y, j.abs() as usize);
      }
      else {
        nx = usize::checked_add(x, *i as usize);
        ny = usize::checked_add(y, *j as usize);
      }
      // Get valid values
      if let (Some(nx), Some(ny)) = (nx, ny) {
        // Ignore invalid positions
        if nx >= 4 || ny >= 4 { continue; }
        // Get positions and apply move
        let mut board = current.board.clone();
        let aux = board[x][y]; board[x][y] = board[nx][ny]; board[nx][ny] = aux;
        let cost = Some(h(&board, &goal));
        // Create the next state
        let next = State::new(board, nmoves, (nx, ny), current.depth + 1, cost);
        // Check if the state was visited
        if visited.contains_key(&next) { continue; }
        // Keep searching
        let state = search(bound, next, goal, visited);
        // Check if the goal was reached
        if state.board == GOAL { return state; }
        // Update minimun depth
        if min == None { min = state.cost; }
        if let (Some(m), Some(c)) = (min, state.cost) {
          if m > c { min = state.cost;}
        }
      }
    }
    // Return invalid solution and a new minimun depth
    State::invnew(min)
}

/// Solves the puzzle by using IDA* algorithm.
fn solve(board: Board, goal: &GoalMap) -> String {
  // Initialize the inicial state
  let blank = find_blank_position(&board);
  let cost: Cost = Some(h(&board, &goal));
  let istate = State::new(board, String::new(), blank, 0, cost);
  // Visited states list
  let mut visited: HashMap<State, String> = HashMap::new();
  // Initialize the bound of search depth
  let mut bound = cost.clone().unwrap();
  loop {
    visited.clear();
    // Search for the goal
    let state = search(bound, istate.clone(), &goal, &mut visited);
    // Check if the solution was found
    if state.board == GOAL { return state.moves; }
    // Check if the cost is infinite
    else if let None = state.cost { println!("None"); return String::new(); }
    // Update bound of search cost
    else if let Some(cost) = state.cost { bound = cost; }
    // Check if the end condition is valid
    if bound >= 50 { println!("50"); return String::new(); }
  }
}

fn main() {
  // Initialize goal map
  let goal_map = goal_map();
  // Get number of test cases
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Error: Unable to read user input.");
  if let Ok(ntc) = input.trim().parse::<usize>() {
    // Run test cases
    for _ in 0..ntc {
      // Read puzzle board
      let board = read_board();
      // Check if the puzzle is solvable
      if is_solvable(&board) {
        println!("{}", solve(board, &goal_map));
      } else {
        println!("This puzzle is not solvable.");
      }
    }
  }
}
