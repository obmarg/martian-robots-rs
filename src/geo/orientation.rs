use super::location::Point;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Orientation {
    North,
    East,
    South,
    West,
}
use Orientation::{East, North, South, West};

impl Orientation {
    pub fn as_point(&self) -> Point {
        match self {
            North => Point { x: 0, y: 1 },
            East => Point { x: 1, y: 0 },
            South => Point { x: 0, y: -1 },
            West => Point { x: -1, y: 0 },
        }
    }
}

pub enum TurnDirection {
    Left,
    Right,
}
use TurnDirection::{Left, Right};

impl Orientation {
    pub fn turn(&self, rhs: TurnDirection) -> Orientation {
        match (self, rhs) {
            (orientation, Left) => match orientation {
                North => West,
                West => South,
                South => East,
                East => North,
            },
            (orientation, Right) => match orientation {
                North => East,
                East => South,
                South => West,
                West => North,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Orientation::*;
    use super::TurnDirection::*;

    #[test]
    fn turns_left() {
        let pairs = [(North, West), (West, South), (South, East), (East, North)];
        for &(before, after) in &pairs {
            assert_eq!(before.turn(Left), after);
        }
    }

    #[test]
    fn turns_right() {
        let pairs = [(North, West), (West, South), (South, East), (East, North)];
        for &(before, after) in &pairs {
            assert_eq!(before.turn(Left), after);
        }
    }
}
