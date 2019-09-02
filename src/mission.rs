use std::collections::HashMap;
use std::collections::HashSet;

use crate::geo::location::Point;
use crate::geo::orientation::Orientation;
use crate::robot::Command;
use crate::robot::Robot;

pub struct Mission {
    pub upper_right: Point,
    scents: HashMap<Point, HashSet<Orientation>>,
}

const ORIGIN: Point = Point { x: 0, y: 0 };

impl Mission {
    pub fn new(upper_right: Point) -> Mission {
        Mission {
            upper_right: upper_right,
            scents: HashMap::new(),
        }
    }

    pub fn dispatch(&mut self, robot: Robot, commands: &[Command]) -> Result<Robot, Robot> {
        commands.iter().try_fold(robot, |r, c| {
            let robot = r.advance(*c);

            if (ORIGIN.x..=self.upper_right.x).contains(&robot.position.x)
                && (ORIGIN.y..=self.upper_right.y).contains(&robot.position.y)
            {
                return Ok(robot);
            }

            if let Some(scent) = self.scents.get(&r.position) {
                if scent.contains(&robot.facing) {
                    return Ok(r);
                }
            }

            self.scents
                .entry(r.position)
                .or_insert(HashSet::new())
                .insert(r.facing);

            Err(r)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Mission;
    use crate::geo::location::Point;
    use crate::geo::orientation::Orientation::{East, North, South, West};
    use crate::robot::Command::{Forward as F, Left as L, Right as R};
    use crate::robot::Robot;

    #[test]
    fn simple_robot() {
        let mut mission = Mission::new(Point { x: 5, y: 3 });
        let robot = Robot {
            position: Point { x: 1, y: 1 },
            facing: East,
        };

        let expected = Ok(Robot {
            position: Point { x: 1, y: 1 },
            facing: East,
        });
        let actual = mission.dispatch(robot, &[R, F, R, F, R, F, R, F]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn robot_is_lost() {
        let mut mission = Mission::new(Point { x: 5, y: 3 });
        let robot = Robot {
            position: Point { x: 3, y: 2 },
            facing: North,
        };

        let expected = Err(Robot {
            position: Point { x: 3, y: 3 },
            facing: North,
        });
        let actual = mission.dispatch(robot, &[F, R, R, F, L, L, F, F, R, R, F, L, L]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn robots_are_clever() {
        let mut mission = Mission::new(Point { x: 5, y: 3 });
        let robot = Robot {
            position: Point { x: 3, y: 2 },
            facing: North,
        };

        let expected = Err(Robot {
            position: Point { x: 3, y: 3 },
            facing: North,
        });
        let actual = mission.dispatch(robot, &[F, R, R, F, L, L, F, F, R, R, F, L, L]);

        assert_eq!(actual, expected);

        let robot = Robot {
            position: Point { x: 0, y: 3 },
            facing: West,
        };

        let expected = Ok(Robot {
            position: Point { x: 2, y: 3 },
            facing: South,
        });
        let actual = mission.dispatch(robot, &[L, L, F, F, F, L, F, L, F, L]);

        assert_eq!(actual, expected);
    }
}
