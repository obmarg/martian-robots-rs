use colored::Colorize;
use std::fmt;

use crate::geo::location::Point;
use crate::geo::orientation::Orientation;
use crate::mission::Outcome;
use crate::robot::{Command, Robot};

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

pub fn outcome(outcome: Outcome) {
    println!("{}", outcome);
}

pub fn verify(expected: Outcome, actual: Outcome) {
    if actual == expected {
        println!("{}", format!("✓ {}", actual).green());
    } else {
        println!(
            "{}",
            format!("⨯ Expected: {}, got: {}", expected, actual).red()
        );
    }
}

// Display support

impl std::fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Orientation::North => 'N',
            Orientation::East => 'E',
            Orientation::South => 'S',
            Orientation::West => 'W',
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
            Command::Left => 'L',
            Command::Right => 'R',
            Command::Forward => 'F',
        };
        write!(f, "{}", text)
    }
}

impl std::fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Outcome::Success(robot) => write!(f, "{}", robot),
            Outcome::Lost(robot) => write!(f, "{} LOST", robot),
        }
    }
}
