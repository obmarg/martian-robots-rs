mod generator;
mod geo;
mod mission;
mod parser;
mod print;
mod robot;

use std::io;

use structopt::StructOpt;

use generator::Generator;
use parser::{MissionOutcomes, MissionPlan};

/// An example solution of the martian robots coding exercise, which can also be used to test implementations.
/// Consumes input from STDIN.
#[derive(StructOpt)]
#[structopt(author = "Viktor Charypar <charypar@gmail.com>", version = "0.2")]
struct Opts {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt)]
enum Command {
    /// Generates pseudo-random robot runs for testing
    Generate(GenerateOpts),
    Verify(VerifyOpts),
}

#[derive(StructOpt)]
struct GenerateOpts {
    /// Only generate a given number of robots
    #[structopt(short = "n")]
    limit: Option<usize>,
    /// Random seed to use
    #[structopt(short, default_value = "12345")]
    seed: u64,
}

#[derive(StructOpt)]
struct VerifyOpts {
    /// Random seed to use
    #[structopt(short, default_value = "12345")]
    seed: u64,
}

fn main() {
    let opts = Opts::from_args();

    let stdin = io::stdin();
    let mut input = stdin.lock();

    match opts.cmd {
        Some(Command::Generate(opts)) => {
            let gen = Generator::new(opts.seed);

            match opts.limit {
                Some(limit) => print::plan(gen.upper_right, gen.take(limit)),
                None => print::plan(gen.upper_right, gen),
            };
        }
        Some(Command::Verify(opts)) => {
            let actual_outcomes = MissionOutcomes::read(&mut input);
            let expected_outcomes = Generator::new(opts.seed).mission();

            print::checks(expected_outcomes.zip(actual_outcomes));
        }
        None => {
            match MissionPlan::read(&mut input) {
                Ok(plan) => {
                    let mission = plan.mission();
                    print::outcomes(mission)
                }
                Err(msg) => eprintln!("{}", msg),
            };
        }
    }
}
