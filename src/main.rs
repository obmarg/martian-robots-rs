mod geo;
mod mission;
mod parser;
mod robot;

use geo::location::Point;
use geo::orientation::Orientation::{self, East, North, South, West};
use mission::Mission;
use robot::Command::{Forward as F, Left as L, Right as R};
use robot::Robot;
use std::fmt;

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
    let mut mission = Mission::new(Point { x: 5, y: 3 });
    let robot = Robot {
        position: Point { x: 1, y: 1 },
        facing: East,
    };
    match mission.dispatch(robot, &[L, F, L, F, L, F, L, F]) {
        Ok(robot) => println!("{}", robot),
        Err(robot) => println!("{} LOST", robot),
    }

    let robot = Robot {
        position: Point { x: 3, y: 2 },
        facing: North,
    };
    match mission.dispatch(robot, &[F, R, R, F, L, L, F, F, R, R, F, L, L]) {
        Ok(robot) => println!("{}", robot),
        Err(robot) => println!("{} LOST", robot),
    }

    let robot = Robot {
        position: Point { x: 0, y: 3 },
        facing: West,
    };
    match mission.dispatch(robot, &[L, L, F, F, F, L, F, L, F, L]) {
        Ok(robot) => println!("{}", robot),
        Err(robot) => println!("{} LOST", robot),
    }
}
