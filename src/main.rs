mod geo;
mod mission;
mod parser;
mod robot;

use std::fmt;
use std::io;

use geo::orientation::Orientation::{self, East, North, South, West};
use parser::MissionPlan;
use robot::Robot;
use mission::Mission;

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
        write!(f, "{} {} {}", self.position.x, self.position.y, self.facing)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();

    let mut plan = match MissionPlan::read(&mut input) {
        Ok(plan) => plan,
        Err(msg) => return eprintln!("{}", msg)
    };

    let mut mission = Mission::new(plan.upper_right);

    for item in &mut plan {
        match item {
            Ok((robot, commands)) => match mission.dispatch(robot, &commands) {
                Ok(robot) => println!("{}", robot),
                Err(robot) => println!("{} LOST", robot),
            },
            Err(err) => return eprintln!("{}", err)
        }
    }
}
