use std::fmt;

use crate::geo::location::Point;
use crate::geo::orientation::Orientation::{self, East, North, South, West};
use crate::robot::{
  Command::{self, Forward, Left, Right},
  Robot,
};

pub fn robots<I>(stream: I)
where
  I: Iterator<Item = (Robot, Vec<Command>)>,
{
  for (robot, commands) in stream {
    println!("{}", robot);
    println!(
      "{}\n",
      commands
        .iter()
        .map(|c| format!("{}", c))
        .collect::<Vec<String>>()
        .join("")
    );
  }
}

pub fn outcome(result: Result<Robot, Robot>) {
  match result {
    Ok(robot) => println!("{}", robot),
    Err(robot) => println!("{} LOST", robot),
  }
}

// Display support

impl std::fmt::Display for Orientation {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let text = match self {
      North => 'N',
      East => 'E',
      South => 'S',
      West => 'W',
    };
    write!(f, "{}", text)
  }
}

impl std::fmt::Display for Robot {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} {}", self.position, self.facing)
  }
}

impl std::fmt::Display for Point {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} {}", self.x, self.y)
  }
}

impl std::fmt::Display for Command {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let text = match self {
      Left => 'L',
      Right => 'R',
      Forward => 'F',
    };
    write!(f, "{}", text)
  }
}
