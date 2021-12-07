use std::env;
use std::fs;

fn min_fuel(input: &[i64]) -> i64 {
    let mut min_cost = i64::MAX;
    for pos in *input.iter().min().unwrap()..*input.iter().max().unwrap() + 1 {
        let mut cost = 0;
        for sub in input.iter() {
            cost += (sub - pos).abs();
        }
        if cost < min_cost {
            min_cost = cost;
        }
    }
    min_cost
}

fn min_fuel_with_increasing_cost(input: &[i64]) -> i64 {
    let mut min_cost = i64::MAX;
    for pos in *input.iter().min().unwrap()..*input.iter().max().unwrap() + 1 {
        let mut cost = 0;
        for sub in input.iter() {
            let n = (sub - pos).abs();
            cost += n * (n + 1) / 2;
        }
        if cost < min_cost {
            min_cost = cost;
        }
    }
    min_cost
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let input_str = fs::read_to_string(&args[1]).expect("couldn't read file");
    let input = input_str
        .trim()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    println!("{}", min_fuel(&input));
    println!("{}", min_fuel_with_increasing_cost(&input));
}
