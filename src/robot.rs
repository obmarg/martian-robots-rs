use crate::geo::location;
use crate::geo::orientation;

#[derive(PartialEq, Debug)]
pub struct Robot {
    pub position: location::Point,
    pub facing: orientation::Orientation,
}

#[derive(Clone, Copy)]
pub enum Command {
    Left,
    Right,
    Forward,
}

impl Robot {
    pub fn advance(self: Robot, command: Command) -> Robot {
        match command {
            Command::Left => Robot {
                position: self.position,
                facing: self.facing.turn(orientation::TurnDirection::Left),
            },
            Command::Right => Robot {
                position: self.position,
                facing: self.facing.turn(orientation::TurnDirection::Right),
            },
            Command::Forward => Robot {
                position: self.position + self.facing,
                facing: self.facing,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geo::location::Point;
    use crate::geo::orientation::Orientation;

    #[test]
    fn robot_turns_left() {
        let robot = Robot {
            position: Point { x: 1, y: 2 },
            facing: Orientation::East,
        };

        let expected = Robot {
            position: Point { x: 1, y: 2 },
            facing: Orientation::North,
        };
        let actual = robot.advance(Command::Left);
        assert_eq!(actual, expected);
    }

    #[test]
    fn robot_turns_right() {
        let robot = Robot {
            position: Point { x: 1, y: 2 },
            facing: Orientation::East,
        };

        let expected = Robot {
            position: Point { x: 1, y: 2 },
            facing: Orientation::South,
        };
        let actual = robot.advance(Command::Right);
        assert_eq!(actual, expected);
    }

    #[test]

    fn robot_moves_forward() {
        let robot = Robot {
            position: Point { x: 1, y: 2 },
            facing: Orientation::East,
        };

        let expected = Robot {
            position: Point { x: 2, y: 2 },
            facing: Orientation::East,
        };
        let actual = robot.advance(Command::Forward);
        assert_eq!(actual, expected);
    }
}
