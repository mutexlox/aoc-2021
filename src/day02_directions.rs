use std::env;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
enum SubCommand {
    Forward(i64),
    HeightChange(i64),
}

#[derive(Debug, PartialEq, Clone)]
enum SubCommandErr {
    ParseIntErr(ParseIntError),
    InvalidCommand(String),
}

impl FromStr for SubCommand {
    type Err = SubCommandErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 2 {
            return Err(SubCommandErr::InvalidCommand(s.to_string()));
        }
        let dist = match parts[1].parse::<i64>() {
            Err(e) => return Err(SubCommandErr::ParseIntErr(e)),
            Ok(n) => n,
        };
        match parts[0] {
            "forward" => Ok(SubCommand::Forward(dist)),
            "up" => Ok(SubCommand::HeightChange(-dist)),
            "down" => Ok(SubCommand::HeightChange(dist)),
            _ => Err(SubCommandErr::InvalidCommand(s.to_string())),
        }
    }
}

fn compute_position_after(input: &Vec<SubCommand>) -> i64 {
    let mut depth = 0;
    let mut horiz = 0;
    for command in input.iter() {
        match command {
            SubCommand::Forward(x) => horiz += x,
            SubCommand::HeightChange(x) => depth += x,
        }
    }
    depth * horiz
}

fn compute_position_after_with_aim(input: &Vec<SubCommand>) -> i64 {
    let mut depth = 0;
    let mut horiz = 0;
    let mut aim = 0;

    for command in input.iter() {
        match command {
            SubCommand::Forward(x) => {
                horiz += x;
                depth += aim * x;
            }
            SubCommand::HeightChange(x) => aim += x,
        }
    }

    depth * horiz
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let input = fs::read_to_string(&args[1])
        .expect("couldn't read file")
        .trim()
        .split('\n')
        .map(|s| s.parse::<SubCommand>().unwrap())
        .collect::<Vec<_>>();
    println!("{}", compute_position_after(&input));
    println!("{}", compute_position_after_with_aim(&input));
}
