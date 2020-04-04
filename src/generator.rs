use rand::distributions::Standard;
use rand::prelude::*;
use rand::rngs::SmallRng;

use crate::geo::location::Point;
use crate::geo::orientation::Orientation;
use crate::mission::Mission;
use crate::robot::{Command, Robot};

struct Generator {
  pub mission: Mission,
  prng: SmallRng, // a pseudo random number generator
}

impl Generator {
  pub fn new(seed: u64) -> Generator {
    let mut prng = SmallRng::seed_from_u64(seed);
    let mission = Mission::new(Point {
      x: prng.gen_range(1, 51),
      y: prng.gen_range(1, 51),
    });

    Generator { mission, prng }
  }
}

impl Iterator for Generator {
  type Item = (Robot, Vec<Command>);

  fn next(&mut self) -> Option<Self::Item> {
    let rng = &mut self.prng;

    let robot = Robot {
      position: Point {
        x: rng.gen_range(0, self.mission.upper_right.x),
        y: rng.gen_range(0, self.mission.upper_right.y),
      },
      facing: rng.gen(),
    };

    let ncmds = rng.gen_range(1, 100);
    let commands = rng.sample_iter(Standard).take(ncmds).collect();

    Some((robot, commands))
  }
}

// Randomly generated custom types

impl Distribution<Orientation> for Standard {
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Orientation {
    match rng.gen_range(0, 4) {
      0 => Orientation::North,
      1 => Orientation::West,
      2 => Orientation::South,
      _ => Orientation::East,
    }
  }
}

impl Distribution<Command> for Standard {
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Command {
    match rng.gen_range(0, 3) {
      0 => Command::Left,
      1 => Command::Right,
      _ => Command::Forward,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::geo::orientation::Orientation::{East as E, North as N, South as S, West as W};
  use crate::robot::Command::{Forward as F, Left as L, Right as R};

  #[test]
  fn creates_mission() {
    let generator = Generator::new(12345);
    let expected = Point { x: 17, y: 43 };

    assert_eq!(generator.mission.upper_right, expected);
  }

  #[test]
  fn generates_three_robots() {
    let generator = Generator::new(12345);
    let expected = vec![
      (
        Robot {
          position: Point { x: 16, y: 39 },
          facing: S,
        },
        vec![
          R, L, R, L, R, L, R, L, F, F, R, L, R, R, F, R, L, F, L, R, L, F, R, F, R, R, F, L, F, L,
          L, F, L, F, F, L, F, R, L, L, F, R, R, R, L, F, R, L, R, R, R, L, F, R, L, L, F, R,
        ],
      ),
      (
        Robot {
          position: Point { x: 12, y: 32 },
          facing: S,
        },
        vec![
          L, R, F, F, F, R, L, R, L, L, R, L, R, L, F, F, R, F, L, F, L, F, L, R, R, F, F, F, R, F,
          F, F, L, F, R, L, F, F, L, L, F, R, L, R, F, F, F, F, R, R, R, R, R, F, F, L, R, R, L, R,
          F, R, L, R, F, R, F, L, R, L, L, R, L, R,
        ],
      ),
      (
        Robot {
          position: Point { x: 7, y: 42 },
          facing: N,
        },
        vec![
          L, L, R, L, L, R, R, R, R, L, F, F, F, R, L, R, L, F, L, F, L, R, F, R, L, R, L, L, R, F,
          R, R, R, R,
        ],
      ),
    ];

    assert_eq!(generator.take(3).collect::<Vec<_>>(), expected);
  }
}
