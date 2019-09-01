mod geo;
mod robot;

use geo::location::Point;
use geo::orientation::Orientation;
use robot::{Command, Robot};

fn main() {
    println!("Hello, world!");

    let robot = Robot {
        position: Point { x: 1, y: 1 },
        facing: Orientation::East,
    };

    let robot = robot.advance(Command::Right);
    let robot = robot.advance(Command::Forward);
    let robot = robot.advance(Command::Right);
    let robot = robot.advance(Command::Forward);
    let robot = robot.advance(Command::Right);
    let robot = robot.advance(Command::Forward);
    let robot = robot.advance(Command::Right);
    let robot = robot.advance(Command::Forward);

    println!("Final robot {:?}", robot);
}
