mod generator;
mod geo;
mod mission;
mod parser;
mod robot;

use std::fmt;
use std::io;

use clap::{App, Arg, SubCommand};

use generator::Generator;
use geo::location::Point;
use geo::orientation::Orientation::{self, East, North, South, West};
use mission::Mission;
use parser::MissionPlan;
use robot::{
    Command::{self, Forward, Left, Right},
    Robot,
};

const SEED: u64 = 12345;

fn main() {
    let matches = App::new("Martian Robots")
        .version("0.2")
        .author("Viktor Charypar <charypar@gmail.com>")
        .about("An example solution of the martian robots coding exercise, which can also be used to test implementations. Consumes input from STDIN.")
        .subcommand(SubCommand::with_name("generate")
            .about("generates pseudo-random stream of robots")
            .arg(Arg::with_name("limit")
                .short("n")
                .value_name("LIMIT")
                .takes_value(true)))
        .get_matches();

    if let Some(generate) = matches.subcommand_matches("generate") {
        let n = generate.value_of("limit").map(|s| s.parse());
        if let Some(Err(_)) = n {
            eprintln!("Invalid limit");
            return;
        }

        let gen = Generator::new(SEED);
        println!("{}", gen.mission.upper_right);

        match n {
            Some(Ok(limit)) => print(gen.take(limit)),
            None => print(gen),
            _ => panic!(),
        };

        return;
    }

    let stdin = io::stdin();
    let mut input = stdin.lock();

    let mut plan = match MissionPlan::read(&mut input) {
        Ok(plan) => plan,
        Err(msg) => return eprintln!("{}", msg),
    };

    let mut mission = Mission::new(plan.upper_right);

    for item in &mut plan {
        match item {
            Ok((robot, commands)) => match mission.dispatch(robot, &commands) {
                Ok(robot) => println!("{}", robot),
                Err(robot) => println!("{} LOST", robot),
            },
            Err(err) => return eprintln!("{}", err),
        }
    }
}

fn print<I>(stream: I)
where
    I: Iterator<Item = (Robot, Vec<Command>)>,
{
    for (robot, commands) in stream {
        println!("{}", robot);
        println!(
            "{}\n",
            commands
                .iter()
                .map(|c| format!("{}", c))
                .collect::<Vec<String>>()
                .join("")
        );
    }
}

// Display support

impl std::fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            North => 'N',
            East => 'E',
            South => 'S',
            West => 'W',
        };
        write!(f, "{}", text)
    }
}

impl std::fmt::Display for Robot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.position, self.facing)
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Left => 'L',
            Right => 'R',
            Forward => 'F',
        };
        write!(f, "{}", text)
    }
}
