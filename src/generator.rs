use rand::distributions::Standard;
use rand::prelude::*;
use rand::rngs::SmallRng;

use crate::geo::location::Point;
use crate::geo::orientation::Orientation;
use crate::robot::{Command, Robot};

pub struct Generator {
    pub upper_right: Point,
    prng: SmallRng, // a pseudo random number generator
}

impl Generator {
    pub fn new(seed: u64) -> Generator {
        let mut prng = SmallRng::seed_from_u64(seed);
        let upper_right = Point {
            x: prng.gen_range(1, 51),
            y: prng.gen_range(1, 51),
        };

        Generator { upper_right, prng }
    }
}

impl Iterator for Generator {
    type Item = (Robot, Vec<Command>);

    fn next(&mut self) -> Option<Self::Item> {
        let rng = &mut self.prng;

        let robot = Robot {
            position: Point {
                x: rng.gen_range(0, self.upper_right.x),
                y: rng.gen_range(0, self.upper_right.y),
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
    use insta::assert_debug_snapshot;

    #[test]
    fn creates_mission() {
        let generator = Generator::new(12345);
        assert_debug_snapshot!(generator.upper_right);
    }

    #[test]
    fn generates_three_robots() {
        let generator = Generator::new(12345);
        assert_debug_snapshot!(generator.take(3).collect::<Vec<_>>());
    }
}
