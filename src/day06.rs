use std::env;
use std::fs;

fn num_fish_after(input: &[usize], days: i64) -> usize {
    // track number of fish of each age
    let mut fishies = [0; 9];
    for x in input.iter() {
        fishies[*x] += 1;
    }
    for _ in 0..days {
        let zero_count = fishies[0];
        for i in 1..fishies.len() {
            fishies[i - 1] = fishies[i];
        }
        fishies[6] += zero_count;
        fishies[8] = zero_count;
    }

    fishies.iter().sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let input_str = fs::read_to_string(&args[1]).expect("couldn't read file");
    let input = input_str
        .trim()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    println!("{}", num_fish_after(&input, 80));
    println!("{}", num_fish_after(&input, 256));
}
