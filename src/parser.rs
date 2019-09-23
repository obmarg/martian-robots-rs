use combine::parser::char::digit;
use combine::{many1, token, Parser};
use std::io::BufRead;

use crate::geo::location::Point;
use crate::mission::Mission;

pub struct MissionPlan<'a> {
  pub mission: Mission,
  reader: Box<dyn BufRead + 'a>,
}

impl<'a> MissionPlan<'_> {
  #[allow(dead_code)]
  pub fn read<T>(mut input: T) -> MissionPlan<'a>
  where
    T: BufRead + 'a,
  {
    let mut line = String::with_capacity(5); // the largest numbers are "50 50"
    input.read_line(&mut line).unwrap();

    let mut coordinates = many1(digit())
      .map(|s: String| s.parse().unwrap())
      .skip(token(' '))
      .and(many1(digit()).map(|s: String| s.parse().unwrap()))
      .skip(token('\n'));

    let result = coordinates.easy_parse(line.as_str());

    match result {
      Ok(((x, y), _)) => MissionPlan {
        mission: Mission::new(Point { x: x, y: y }),
        reader: Box::new(input),
      },
      Err(err) => panic!("AAAAHHH! {}", err),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::geo::location::Point;
  use crate::mission::Mission;

  #[test]
  fn reads_basic_mission_plan() {
    let input = b"31 24\n";
    let actual = MissionPlan::read(&input[..]).mission;
    let expected = Mission::new(Point { x: 31, y: 24 });

    assert_eq!(actual, expected)
  }
}
