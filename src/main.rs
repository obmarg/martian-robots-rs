mod generator;
mod geo;
mod mission;
mod parser;
mod print;
mod robot;

use std::io;

use structopt::StructOpt;

use generator::Generator;
use mission::Mission;
use parser::MissionPlan;

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

fn main() {
    let opts = Opts::from_args();

    if let Some(Command::Generate(opts)) = opts.cmd {
        let gen = Generator::new(opts.seed);

        println!("{}", gen.upper_right);
        match opts.limit {
            Some(limit) => print::robots(gen.take(limit)),
            None => print::robots(gen),
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
            Ok((robot, commands)) => print::outcome(mission.dispatch(robot, &commands)),
            Err(err) => return eprintln!("{}", err),
        }
    }
}
