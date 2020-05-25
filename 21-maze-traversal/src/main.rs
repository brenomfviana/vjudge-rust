use std::io;
use std::collections::HashMap;

// Robot directions
const N: usize = 0; // North
const S: usize = 1; // South
const W: usize = 2; // West
const E: usize = 3; // East

// Robot movements
const R: char = 'R'; // Right
const L: char = 'L'; // Left
const F: char = 'F'; // Move forward
const Q: char = 'Q'; // Quit

// Alias for graph nodes and edges
type Node = usize;
type Direction = usize;
type Position = (usize, usize);
type Maze = Vec<Vec<char>>;
type Way = HashMap<Direction, Node>;
type AdjMtx = HashMap<Node, Way>;
type PositionNode = HashMap<Position, Node>;
type NodePosition = HashMap<Node, Position>;

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
  pub o: usize, // Orientation
}

impl Robot {
  /// Create and return a new robot.
  fn new(p: Position, o: usize) -> Robot { Robot { p, o } }

  /// Read and perform the commands.
  fn run_commands(&mut self, maze: &Graph) {
    // Read robot moves and move it
    loop {
      // Read the input
      let mut input = String::new();
      io::stdin().read_line(&mut input)
        .expect("Error: Unable to read user input.");
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
            if let Some(_) = adjlst.get(&self.o) {
              self.p = maze.ndpst[&maze.adjmt[&maze.pstnd[&self.p]][&self.o]];
            }
          }
        }
        // Turn right
        if c == R {
          match self.o {
            N => self.o = E,
            S => self.o = W,
            W => self.o = N,
            E => self.o = S,
            _ => (),
          }
        }
        // Turn left
        if c == L {
          match self.o {
            N => self.o = W,
            S => self.o = E,
            W => self.o = S,
            E => self.o = N,
            _ => (),
          }
        }
      }
    }
  }
}

/// Read the maze and return the corresponding graph.
fn read_maze() -> Graph {
  // Read the input
  let mut input = String::new();
  io::stdin().read_line(&mut input)
    .expect("Error: Unable to read user input.");
  // Get the number of row and col of the maze
  let values: Vec<usize> = input.split(" ")
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
    // Read the input
    let mut input = String::new();
    io::stdin().read_line(&mut input)
      .expect("Error: Unable to read user input.");
    // Maze lines
    let line: Vec<char> = input.trim().chars().collect();
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
            way.insert(N, grph.pstnd[&p2]);
          } else {
            let mut way = Way::new(); way.insert(N, grph.pstnd[&p2]);
            grph.adjmt.insert(grph.pstnd[&p1], way);
          }
          // Check if the `from` node was already added (p2 -> p1)
          if let Some(way) = grph.adjmt.get_mut(&grph.pstnd[&p2]) {
            way.insert(S, grph.pstnd[&p1]);
          } else {
            let mut way = Way::new(); way.insert(S, grph.pstnd[&p1]);
            grph.adjmt.insert(grph.pstnd[&p2], way);
          }
        }
        if j > 0 && maze[i][j - 1] == ' ' {
          let (p1, p2) = ((i + 1, j + 1), (i + 1, j));
          // Check if the `from` node was already added (p1 -> p2)
          if let Some(way) = grph.adjmt.get_mut(&grph.pstnd[&p1]) {
            way.insert(W, grph.pstnd[&p2]);
          } else {
            let mut way = Way::new(); way.insert(W, grph.pstnd[&p2]);
            grph.adjmt.insert(grph.pstnd[&p1], way);
          }
          // Check if the `from` node was already added (p2 -> p1)
          if let Some(way) = grph.adjmt.get_mut(&grph.pstnd[&p2]) {
            way.insert(E, grph.pstnd[&p1]);
          } else {
            let mut way = Way::new(); way.insert(E, grph.pstnd[&p1]);
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
  // Read the input
  let mut input = String::new();
  io::stdin().read_line(&mut input)
    .expect("Error: Unable to read user input.");
  // Get the row and col position of the robot
  let values: Vec<usize> = input.split(" ")
    .map(|s| s.trim().parse::<usize>())
    .filter_map(Result::ok).collect();
  // Check if the read data is invalid
  if values.len() != 2 { panic!("Invalid file."); }
  let (row, col) = (values[0], values[1]);
  // Return the read robot
  Robot::new((row, col), N)
}

fn main() {
  // Get number of test cases
  let mut input = String::new();
  io::stdin().read_line(&mut input)
    .expect("Error: Unable to read user input.");
  let itc = input.trim().parse::<usize>();
  // Read an empty line
  let mut input = String::new();
  io::stdin().read_line(&mut input)
    .expect("Error: Unable to read user input.");
  // Check if the number of test cases was read
  if let Ok(itc) = itc {
    // Get number of test cases
    let ntc = itc;
    let mut itc = Some(itc - 1);
    // Run test cases
    while itc != None {
      // Read maze and robot
      let maze = read_maze();
      let mut robot = read_robot();
      // Move robot on the maze
      robot.run_commands(&maze);
      // Check if the current iteration is the first one
      if itc != Some(ntc - 1) { println!(""); }
      // Print last position and orientation of the robot
      print!("{} {} ", robot.p.0, robot.p.1);
      if robot.o == N { print!("N"); }
      if robot.o == S { print!("S"); }
      if robot.o == W { print!("W"); }
      if robot.o == E { print!("E"); }
      println!("");
      // Next test case
      if let Some(i) = itc { itc = i.checked_sub(1); }
    }
  }
}
