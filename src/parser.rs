use std::io::Read;
use std::str;

use combine::error::ParseError;
use combine::parser::byte::{bytes, digit, space, spaces};
use combine::stream::buffered;
use combine::stream::position;
use combine::stream::read;
use combine::stream::Stream;
use combine::{eof, many1, one_of, optional, skip_many, EasyParser, Parser};

use crate::geo::location::Point;
use crate::geo::orientation::Orientation;
use crate::mission::Outcome;
use crate::robot::{Command, Robot};

pub struct MissionPlan<'a, R>
where
    R: Read,
{
    pub upper_right: Point,
    stream:
        Box<buffered::Stream<position::Stream<read::Stream<&'a mut R>, position::IndexPositioner>>>,
}

pub struct MissionOutcomes<'a, R>
where
    R: Read,
{
    stream:
        Box<buffered::Stream<position::Stream<read::Stream<&'a mut R>, position::IndexPositioner>>>,
}

// Parses an X, Y point written as two integers separated by whitespace
fn point<Input>() -> impl Parser<Input, Output = Point>
where
    Input: Stream<Token = u8>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (many1(digit()), spaces(), many1(digit())).map(|(x, _, y): (Vec<u8>, _, Vec<u8>)| Point {
        x: str::from_utf8(&x).unwrap().parse().unwrap(),
        y: str::from_utf8(&y).unwrap().parse().unwrap(),
    })
}

// Parses an orientation written as a single letter N, E, S or W
fn orientation<Input>() -> impl Parser<Input, Output = Orientation>
where
    Input: Stream<Token = u8>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    one_of("NESW".bytes()).map(|token: u8| match token as char {
        'N' => Orientation::North,
        'E' => Orientation::East,
        'S' => Orientation::South,
        'W' => Orientation::West,
        _ => panic!(),
    })
}

// Parses a contiguous series of commands L, R or F
fn commands<Input>() -> impl Parser<Input, Output = Vec<Command>>
where
    Input: Stream<Token = u8>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    many1(one_of("LRF".bytes())).map(|cmds: Vec<u8>| {
        cmds.iter()
            .map(|cmd| match *cmd as char {
                'L' => Command::Left,
                'R' => Command::Right,
                'F' => Command::Forward,
                _ => panic!(),
            })
            .collect()
    })
}

// Parses a robot definition followed by instructions
fn robot<Input>() -> impl Parser<Input, Output = (Robot, Vec<Command>)>
where
    Input: Stream<Token = u8>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    point()
        .skip(spaces())
        .and(orientation())
        .skip(spaces()) // spaces covers new lines
        .and(commands())
        .skip(space())
        .map(|((point, orientation), commands)| {
            (
                Robot {
                    position: point,
                    facing: orientation,
                },
                commands,
            )
        })
}

// Parses an outcome of a robot run, e.g. '3 3 N', or '5 2 E LOST'
fn outcome<Input>() -> impl Parser<Input, Output = Outcome>
where
    Input: Stream<Token = u8, Range = &'static [u8]>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    point()
        .skip(spaces())
        .and(orientation())
        .skip(spaces())
        .and(optional(bytes(&b"LOST"[..])).skip(spaces()))
        .map(|((position, orientation), lost)| match lost {
            None => Outcome::Success(Robot {
                position,
                facing: orientation,
            }),
            Some(_) => Outcome::Lost(Robot {
                position,
                facing: orientation,
            }),
        })
}

impl<R> MissionPlan<'_, R>
where
    R: Read,
{
    pub fn read(input: &mut R) -> Result<MissionPlan<R>, String> {
        // Should return Result
        let mut stream = buffered::Stream::new(position::Stream::new(read::Stream::new(input)), 1);
        let upper_right;

        {
            let point = skip_many(space())
                .and(point())
                .skip(spaces())
                .parse(&mut stream);

            upper_right = match point {
                Ok(((_, point), _)) => point,
                Err(err) => return Err(format!("Expected grid size. {}", err)), // this could be improved
            };
        } // return borrowed stream

        Ok(MissionPlan {
            upper_right: upper_right,
            stream: Box::new(stream), // ...so it can be moved here
        })
    }
}

impl<R> Iterator for MissionPlan<'_, R>
where
    R: Read,
{
    type Item = Result<(Robot, Vec<Command>), String>;

    fn next(&mut self) -> Option<Self::Item> {
        let stream = self.stream.as_mut();
        let robot = skip_many(space())
            .and(
                robot().map(|r| Some(r)).or(eof().map(|()| None)), // an expected end of input
            )
            .easy_parse(stream);

        match robot {
            Ok(((_, None), _)) => None,
            Ok(((_, Some(robot)), _)) => Some(Ok(robot)),
            Err(error) => {
                let human_error = error
                    .map_token(|t| t as char)
                    .map_range(|r| std::str::from_utf8(r).unwrap());
                Some(Err(format!("{}", human_error)))
            }
        }
    }
}

impl<R> MissionOutcomes<'_, R>
where
    R: Read,
{
    pub fn read(input: &mut R) -> MissionOutcomes<R> {
        // Should return Result
        let stream = buffered::Stream::new(position::Stream::new(read::Stream::new(input)), 1);

        MissionOutcomes {
            stream: Box::new(stream),
        }
    }
}

impl<R> Iterator for MissionOutcomes<'_, R>
where
    R: Read,
{
    type Item = Result<Outcome, String>;

    fn next(&mut self) -> Option<Self::Item> {
        let stream = self.stream.as_mut();
        let outcome = skip_many(space())
            .and(
                outcome().map(|r| Some(r)).or(eof().map(|()| None)), // expected EOF
            )
            .easy_parse(stream);

        match outcome {
            // End of stream
            Ok(((_, None), _)) => None,
            // Successfully parsed outcome
            Ok(((_, Some(outcome)), _)) => Some(Ok(outcome)),
            // Parse error
            Err(error) => {
                let human_error = error
                    .map_token(|t| t as char)
                    .map_range(|r| std::str::from_utf8(r).unwrap());
                Some(Err(format!("{}", human_error)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use combine;
    use combine::stream::position;
    use std::io::Cursor;

    use crate::geo::location::Point;
    use crate::robot::Command::{Forward as F, Left as L, Right as R};

    #[test]
    fn recognises_a_simple_point() {
        let input = b"3 4";
        let (actual, _) = point().parse(position::Stream::new(&input[..])).unwrap();
        let expected = Point { x: 3, y: 4 };

        assert_eq!(actual, expected)
    }

    #[test]
    fn recognises_a_point_with_extra_whitespace() {
        let input = b"3  \t 4";
        let (actual, _) = point().parse(position::Stream::new(&input[..])).unwrap();
        let expected = Point { x: 3, y: 4 };

        assert_eq!(actual, expected)
    }

    #[test]
    fn does_not_recognise_a_point_with_whitespaces_in_front() {
        let input = b" 3  \t 4";
        let actual = point().parse(position::Stream::new(&input[..]));
        let expected = Err(combine::error::UnexpectedParse::Unexpected);

        assert_eq!(actual, expected);
    }

    #[test]
    fn recognises_commands() {
        let input = b"LRFFLFR";
        let (actual, _) = commands().parse(position::Stream::new(&input[..])).unwrap();
        let expected = vec![L, R, F, F, L, F, R];

        assert_eq!(actual, expected)
    }

    #[test]
    fn recognises_a_robot() {
        let input = b"4  5  W\n\r  LRFFLFR\n"; // we don't discriminate against whitespace
        let (actual, _) = robot().parse(position::Stream::new(&input[..])).unwrap();
        let expected = (
            Robot {
                position: Point { x: 4, y: 5 },
                facing: Orientation::West,
            },
            vec![L, R, F, F, L, F, R],
        );

        assert_eq!(actual, expected)
    }

    #[test]
    fn recognises_a_positive_outcome() {
        let input = b"4  5  W\n";
        let (actual, _) = outcome().parse(position::Stream::new(&input[..])).unwrap();
        let expected = Outcome::Success(Robot {
            position: Point { x: 4, y: 5 },
            facing: Orientation::West,
        });

        assert_eq!(actual, expected)
    }

    #[test]
    fn recognises_a_negative_outcome() {
        let input = b"4  5  E   LOST\n";
        let outcome = outcome().parse(position::Stream::new(&input[..]));
        let expected = Outcome::Lost(Robot {
            position: Point { x: 4, y: 5 },
            facing: Orientation::East,
        });

        match outcome {
            Ok((actual, _)) => assert_eq!(actual, expected),
            Err(msg) => panic!("{:?}", msg),
        }
    }

    #[test]
    fn reads_upper_right() {
        let mut input = Cursor::new("  31 24\n");

        let actual = MissionPlan::read(&mut input).unwrap().upper_right;
        let expected = Point { x: 31, y: 24 };

        assert_eq!(actual, expected)
    }

    #[test]
    fn reads_one_robot() {
        let mut input = Cursor::new("  31 24\n   1 1 E\nLFLFLFLF\n");

        let actual = MissionPlan::read(&mut input).unwrap().next();
        let expected = Some(Ok((
            Robot {
                position: Point { x: 1, y: 1 },
                facing: Orientation::East,
            },
            vec![L, F, L, F, L, F, L, F],
        )));

        assert_eq!(actual, expected)
    }

    #[test]
    fn collects_three_robots() {
        let mut input =
            Cursor::new("31 24\n1 1 E\nLFLFLFLF\n\n3 2 N\nFRRFLLFFRRFLL\n\n0 3 W\nLLFFFLFLFL\n");

        let plan = MissionPlan::read(&mut input).unwrap();
        let actual = plan.collect::<Vec<_>>();
        let expected = vec![
            Ok((
                Robot {
                    position: Point { x: 1, y: 1 },
                    facing: Orientation::East,
                },
                vec![L, F, L, F, L, F, L, F],
            )),
            Ok((
                Robot {
                    position: Point { x: 3, y: 2 },
                    facing: Orientation::North,
                },
                vec![F, R, R, F, L, L, F, F, R, R, F, L, L],
            )),
            Ok((
                Robot {
                    position: Point { x: 0, y: 3 },
                    facing: Orientation::West,
                },
                vec![L, L, F, F, F, L, F, L, F, L],
            )),
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn reads_one_outcome() {
        let mut input = Cursor::new("  22 11 E LOST\n");

        let actual = MissionOutcomes::read(&mut input).next();
        let expected = Some(Ok(Outcome::Lost(Robot {
            position: Point { x: 22, y: 11 },
            facing: Orientation::East,
        })));

        assert_eq!(actual, expected)
    }

    #[test]
    fn reads_three_outcomes() {
        let mut input = Cursor::new("  1 2 W\n3 3 N LOST\n5 2 S");

        let actual = MissionOutcomes::read(&mut input).collect::<Vec<_>>();
        let expected = vec![
            Ok(Outcome::Success(Robot {
                position: Point { x: 1, y: 2 },
                facing: Orientation::West,
            })),
            Ok(Outcome::Lost(Robot {
                position: Point { x: 3, y: 3 },
                facing: Orientation::North,
            })),
            Ok(Outcome::Success(Robot {
                position: Point { x: 5, y: 2 },
                facing: Orientation::South,
            })),
        ];

        assert_eq!(actual, expected)
    }
}
