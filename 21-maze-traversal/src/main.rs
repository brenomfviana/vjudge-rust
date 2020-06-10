use std::io;
use std::collections::HashMap;

// Robot directions
#[derive(Debug, Eq, PartialEq, Hash)]
enum Direction {
  N = 0, // North
  S = 1, // South
  W = 2, // West
  E = 3, // East
}

// Robot movements
const R: char = 'R'; // Right
const L: char = 'L'; // Left
const F: char = 'F'; // Move forward
const Q: char = 'Q'; // Quit

// Alias for graph nodes and edges
type Node = usize;
// type Direction = usize;
type Position = (usize, usize);
type Maze = Vec<Vec<char>>;
type Way = HashMap<Direction, Node>;
type AdjMtx = HashMap<Node, Way>;
type PositionNode = HashMap<Position, Node>;
type NodePosition = HashMap<Node, Position>;

/// Reads a user input line.
fn read_line() -> String {
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Error: Unable to read user input.");
  input
}

/// Graph struct
#[derive(Debug)]
struct Graph {
  pub adjmt: AdjMtx,
  pub ndpst: NodePosition,
  pub pstnd: PositionNode,
}

impl Graph {
  /// Create and return a new robot.
  fn new() -> Graph {
    Graph {
      adjmt: HashMap::new(), ndpst: HashMap::new(), pstnd: HashMap::new()
    }
  }
}

/// Robot struct
#[derive(Debug)]
struct Robot {
  pub p: Position, // Position
  pub o: Direction, // Orientation
}

impl Robot {
  /// Create and return a new robot.
  fn new(p: Position, o: Direction) -> Robot { Robot { p, o } }

  /// Read and perform the commands.
  fn run_commands(&mut self, maze: &Graph) {
    // Read robot moves and move it
    loop {
      // Read the input
      let input = read_line();
      // Check if the end condition was reached
      if input.trim() == "" { break; }
      // Get the commands
      let commands: Vec<char> = input.split("")
        .map(|s| s.trim().parse::<char>())
        .filter_map(Result::ok).collect();
      // Run commands
      for c in commands {
        // Stop robot
        if c == Q { break; }
        // Move forward
        if c == F {
          // Get the adjacency list of the robot position
          if let Some(adjlst) = &maze.adjmt.get(&maze.pstnd[&self.p]) {
            if adjlst.get(&self.o).is_some() {
              self.p = maze.ndpst[&maze.adjmt[&maze.pstnd[&self.p]][&self.o]];
            }
          }
        }
        // Turn right
        if c == R {
          match self.o {
            Direction::N => self.o = Direction::E,
            Direction::S => self.o = Direction::W,
            Direction::W => self.o = Direction::N,
            Direction::E => self.o = Direction::S,
          }
        }
        // Turn left
        if c == L {
          match self.o {
            Direction::N => self.o = Direction::W,
            Direction::S => self.o = Direction::E,
            Direction::W => self.o = Direction::S,
            Direction::E => self.o = Direction::N,
          }
        }
      }
    }
  }
}

/// Read the maze and return the corresponding graph.
fn read_maze() -> Graph {
  // Get the number of row and col of the maze
  let values: Vec<usize> = read_line().split(' ')
    .map(|s| s.trim().parse::<usize>())
    .filter_map(Result::ok).collect();
  // Check if the read data is invalid
  if values.len() != 2 { panic!("Invalid file."); }
  let (row, col) = (values[0], values[1]);
  // Node ID counter
  let mut id: usize = 1;
  // Read maze
  let (mut maze, mut grph): (Maze, Graph) = (vec![], Graph::new());
  // Read
  for i in 0..row {
    // Maze lines
    let line: Vec<char> = read_line().trim().chars().collect();
    // Check if the read line is invalid
    if line.len() != col { panic!("Invalid file."); }
    // Read the maze row
    maze.push(line);
    // Create the graph
    for (j, c) in maze[i].iter().enumerate() {
      // If the cell is passable then add it to the maze
      if *c == ' ' {
        let point = (i + 1, j + 1);
        grph.pstnd.insert(point, id); grph.ndpst.insert(id, point);
        id += 1;
        // Connect to the read node
        if i > 0 && maze[i - 1][j] == ' ' {
          // Convert indexes to points
          let (p1, p2) = ((i + 1, j + 1), (i, j + 1));
          // Check if the `from` node was already added (p1 -> p2)
          if let Some(way) = grph.adjmt.get_mut(&grph.pstnd[&p1]) {
            way.insert(Direction::N, grph.pstnd[&p2]);
          } else {
            let mut way = Way::new(); way.insert(Direction::N, grph.pstnd[&p2]);
            grph.adjmt.insert(grph.pstnd[&p1], way);
          }
          // Check if the `from` node was already added (p2 -> p1)
          if let Some(way) = grph.adjmt.get_mut(&grph.pstnd[&p2]) {
            way.insert(Direction::S, grph.pstnd[&p1]);
          } else {
            let mut way = Way::new(); way.insert(Direction::S, grph.pstnd[&p1]);
            grph.adjmt.insert(grph.pstnd[&p2], way);
          }
        }
        if j > 0 && maze[i][j - 1] == ' ' {
          let (p1, p2) = ((i + 1, j + 1), (i + 1, j));
          // Check if the `from` node was already added (p1 -> p2)
          if let Some(way) = grph.adjmt.get_mut(&grph.pstnd[&p1]) {
            way.insert(Direction::W, grph.pstnd[&p2]);
          } else {
            let mut way = Way::new(); way.insert(Direction::W, grph.pstnd[&p2]);
            grph.adjmt.insert(grph.pstnd[&p1], way);
          }
          // Check if the `from` node was already added (p2 -> p1)
          if let Some(way) = grph.adjmt.get_mut(&grph.pstnd[&p2]) {
            way.insert(Direction::E, grph.pstnd[&p1]);
          } else {
            let mut way = Way::new(); way.insert(Direction::E, grph.pstnd[&p1]);
            grph.adjmt.insert(grph.pstnd[&p2], way);
          }
        }
      }
    }
  }
  // Return maze
  grph
}

/// Read and return a robot.
fn read_robot() -> Robot {
  // Get the row and col position of the robot
  let values: Vec<usize> = read_line().split(' ')
    .map(|s| s.trim().parse::<usize>())
    .filter_map(Result::ok).collect();
  // Check if the read data is invalid
  if values.len() != 2 { panic!("Invalid file."); }
  let (row, col) = (values[0], values[1]);
  // Return the read robot
  Robot::new((row, col), Direction::N)
}

fn main() {
  // Read the number of test cases
  let ntc = read_line().trim().parse::<usize>()
    .expect("Invalid number of test cases.");
  // Read the number of test cases
  let mut itc = Some(ntc - 1);
  // Read an empty line
  let _ = read_line();
  // Run test cases
  while itc != None {
    // Read maze and robot
    let maze = read_maze();
    let mut robot = read_robot();
    // Move robot on the maze
    robot.run_commands(&maze);
    // Check if the current iteration is the first one
    if itc != Some(ntc - 1) { println!(); }
    // Print last position and orientation of the robot
    print!("{} {} ", robot.p.0, robot.p.1);
    if robot.o == Direction::N { print!("N"); }
    if robot.o == Direction::S { print!("S"); }
    if robot.o == Direction::W { print!("W"); }
    if robot.o == Direction::E { print!("E"); }
    println!();
    // Next test case
    if let Some(i) = itc { itc = i.checked_sub(1); }
  }
}
