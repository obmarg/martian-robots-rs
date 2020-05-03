use colored::Colorize;
use itertools::Itertools;
use std::fmt;

use crate::geo::location::Point;
use crate::geo::orientation::Orientation;
use crate::mission::Outcome;
use crate::robot::{Command, Robot};

pub fn plan<I>(upper_right: Point, stream: I)
where
    I: Iterator<Item = (Robot, Vec<Command>)>,
{
    println!("{}", upper_right);
    for (robot, commands) in stream {
        println!("{}\n{}\n", robot, commands.iter().format(""));
    }
}

pub fn checks<I>(stream: I)
where
    I: Iterator<Item = (Outcome, Result<Outcome, String>)>,
{
    for (expected, actual) in stream {
        if let Err(msg) = actual {
            return eprintln!("{}", msg);
        }

        let actual = actual.unwrap();

        if actual == expected {
            println!("{}", format!("✓ {}", actual).green());
        } else {
            let err = format!("⨯ Expected: {}, got: {}", expected, actual).red();
            println!("{}", err);
        }
    }
}

pub fn outcomes<I>(stream: I)
where
    I: Iterator<Item = Result<Outcome, String>>,
{
    for item in stream {
        match item {
            Ok(outcome) => println!("{}", outcome),
            Err(msg) => return eprintln!("{}", msg),
        }
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
