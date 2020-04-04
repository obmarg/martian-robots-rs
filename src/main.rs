mod generator;
mod geo;
mod mission;
mod parser;
mod print;
mod robot;

use std::io;

use clap::{App, Arg, SubCommand};

use generator::Generator;
use mission::Mission;
use parser::MissionPlan;

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

        println!("{}", gen.upper_right);
        match n {
            Some(Ok(limit)) => print::robots(gen.take(limit)),
            None => print::robots(gen),
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
            Ok((robot, commands)) => print::outcome(mission.dispatch(robot, &commands)),
            Err(err) => return eprintln!("{}", err),
        }
    }
}
