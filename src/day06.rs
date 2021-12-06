use std::env;
use std::fs;

fn num_fish_after(input: &[i64], days: i64) -> usize {
    let mut fishies = input.to_vec();
    for _ in 0..days {
        let mut to_add = 0;
        for fish in fishies.iter_mut() {
            if *fish == 0 {
                *fish = 6;
                to_add += 1;
            } else {
                *fish -= 1;
            }
        }
        for _ in 0..to_add {
            fishies.push(8);
        }
    }

    fishies.len()
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
    println!("{}", num_fish_after(&input, 80));
    println!("{}", num_fish_after(&input, 256));
}
