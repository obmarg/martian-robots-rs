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

pub struct Mission {
    pub upper_right: Point,
    scents: HashMap<Point, HashSet<Orientation>>,
}

const ORIGIN: Point = Point { x: 0, y: 0 };

impl Mission {
    pub fn run<S: SourceItem>(
        upper_right: Point,
        source: impl IntoIterator<Item = S>,
    ) -> impl Iterator<Item = S::Output> {
        let mut mission = Mission {
            upper_right,
            scents: HashMap::new(),
        };
        source.into_iter().map(move |d| d.dispatch(&mut mission))
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

pub trait SourceItem: Sized {
    type Output;

    fn dispatch(self, mission: &mut Mission) -> Self::Output;
}

impl SourceItem for (Robot, Vec<Command>) {
    type Output = Outcome;

    fn dispatch(self, mission: &mut Mission) -> Self::Output {
        let (robot, commands) = self;
        mission.dispatch(robot, commands.as_ref())
    }
}

impl SourceItem for Result<(Robot, Vec<Command>), String> {
    type Output = Result<Outcome, String>;

    fn dispatch(self, mission: &mut Mission) -> Self::Output {
        self.map(|(robot, commands)| mission.dispatch(robot, commands.as_ref()))
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
        let robot = Robot {
            position: Point { x: 1, y: 1 },
            facing: East,
        };

        let expected = Outcome::Success(Robot {
            position: Point { x: 1, y: 1 },
            facing: East,
        });

        let actual: Vec<_> = Mission::run(
            Point { x: 5, y: 3 },
            vec![(robot, vec![R, F, R, F, R, F, R, F])],
        )
        .collect();

        assert_eq!(actual, vec![expected]);
    }

    #[test]
    fn robot_is_lost() {
        let robot = Robot {
            position: Point { x: 3, y: 2 },
            facing: North,
        };

        let expected = Outcome::Lost(Robot {
            position: Point { x: 3, y: 3 },
            facing: North,
        });

        let actual: Vec<_> = Mission::run(
            Point { x: 5, y: 3 },
            vec![(robot, vec![F, R, R, F, L, L, F, F, R, R, F, L, L])],
        )
        .collect();

        assert_eq!(actual, vec![expected]);
    }

    #[test]
    fn robots_are_clever() {
        let mission = vec![
            (
                Robot {
                    position: Point { x: 3, y: 2 },
                    facing: North,
                },
                vec![F, R, R, F, L, L, F, F, R, R, F, L, L],
            ),
            (
                Robot {
                    position: Point { x: 0, y: 3 },
                    facing: West,
                },
                vec![L, L, F, F, F, L, F, L, F, L],
            ),
        ];

        let expected = vec![
            Outcome::Lost(Robot {
                position: Point { x: 3, y: 3 },
                facing: North,
            }),
            Outcome::Success(Robot {
                position: Point { x: 2, y: 3 },
                facing: South,
            }),
        ];

        let actual: Vec<_> = Mission::run(Point { x: 5, y: 3 }, mission).collect();

        assert_eq!(actual, expected);
    }
}
