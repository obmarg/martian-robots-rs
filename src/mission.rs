use std::collections::HashMap;
use std::collections::HashSet;

use crate::geo::location::Point;
use crate::geo::orientation::Orientation;
use crate::robot::Command;
use crate::robot::Robot;

#[derive(Debug, PartialEq)]
pub enum Outcome {
    Success(Robot),
    Lost(Robot),
}

pub struct Mission<I, X>
where
    I: Iterator<Item = X>,
{
    pub upper_right: Point,
    source: I,
    scents: HashMap<Point, HashSet<Orientation>>,
}

const ORIGIN: Point = Point { x: 0, y: 0 };

impl<I, SourceItem> Mission<I, SourceItem>
where
    I: Iterator<Item = SourceItem>,
{
    pub fn new(upper_right: Point, source: I) -> Mission<I, SourceItem> {
        Mission {
            upper_right: upper_right,
            source: source,
            scents: HashMap::new(),
        }
    }

    pub fn dispatch(&mut self, robot: Robot, commands: &[Command]) -> Outcome {
        let outcome = commands.iter().try_fold(robot, |r, c| {
            let robot = r.advance(*c);

            if (ORIGIN.x..=self.upper_right.x).contains(&robot.position.x)
                && (ORIGIN.y..=self.upper_right.y).contains(&robot.position.y)
            {
                // moved robot is still on the grid, commit
                return Ok(robot);
            }

            // moved robot would be off the grid...
            match self.scents.get(&r.position) {
                // ...but previous robot has left a scent, so we'll ignore the move
                Some(scent) if scent.contains(&robot.facing) => return Ok(r),
                // ...and it's lost, but not before leaving a scent in its wake
                _ => {
                    self.scents
                        .entry(r.position)
                        .or_insert(HashSet::new())
                        .insert(r.facing);
                    Err(r)
                }
            }
        });

        match outcome {
            Ok(robot) => Outcome::Success(robot),
            Err(robot) => Outcome::Lost(robot),
        }
    }
}

// Running a mission with a reliable source
impl<I> std::iter::Iterator for Mission<I, (Robot, Vec<Command>)>
where
    I: Iterator<Item = (Robot, Vec<Command>)>,
{
    type Item = Outcome;

    fn next(&mut self) -> Option<Self::Item> {
        self.source
            .next()
            .map(|(robot, commands)| self.dispatch(robot, commands.as_ref()))
    }
}

// Running a mission with a unreliable source
impl<I> std::iter::Iterator for Mission<I, Result<(Robot, Vec<Command>), String>>
where
    I: Iterator<Item = Result<(Robot, Vec<Command>), String>>,
{
    type Item = Result<Outcome, String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.source
            .next()
            .map(|item| item.map(|(robot, commands)| self.dispatch(robot, commands.as_ref())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geo::location::Point;
    use crate::geo::orientation::Orientation::{East, North, South, West};
    use crate::robot::Command::{Forward as F, Left as L, Right as R};
    use crate::robot::Robot;

    #[test]
    fn simple_robot() {
        let mut mission: Mission<_, (Robot, Vec<Command>)> =
            Mission::new(Point { x: 5, y: 3 }, Vec::new().into_iter());
        let robot = Robot {
            position: Point { x: 1, y: 1 },
            facing: East,
        };

        let expected = Outcome::Success(Robot {
            position: Point { x: 1, y: 1 },
            facing: East,
        });
        let actual = mission.dispatch(robot, &[R, F, R, F, R, F, R, F]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn robot_is_lost() {
        let mut mission: Mission<_, (Robot, Vec<Command>)> =
            Mission::new(Point { x: 5, y: 3 }, Vec::new().into_iter());
        let robot = Robot {
            position: Point { x: 3, y: 2 },
            facing: North,
        };

        let expected = Outcome::Lost(Robot {
            position: Point { x: 3, y: 3 },
            facing: North,
        });
        let actual = mission.dispatch(robot, &[F, R, R, F, L, L, F, F, R, R, F, L, L]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn robots_are_clever() {
        let mut mission: Mission<_, (Robot, Vec<Command>)> =
            Mission::new(Point { x: 5, y: 3 }, Vec::new().into_iter());
        let robot = Robot {
            position: Point { x: 3, y: 2 },
            facing: North,
        };

        let expected = Outcome::Lost(Robot {
            position: Point { x: 3, y: 3 },
            facing: North,
        });
        let actual = mission.dispatch(robot, &[F, R, R, F, L, L, F, F, R, R, F, L, L]);

        assert_eq!(actual, expected);

        let robot = Robot {
            position: Point { x: 0, y: 3 },
            facing: West,
        };

        let expected = Outcome::Success(Robot {
            position: Point { x: 2, y: 3 },
            facing: South,
        });
        let actual = mission.dispatch(robot, &[L, L, F, F, F, L, F, L, F, L]);

        assert_eq!(actual, expected);
    }
}
