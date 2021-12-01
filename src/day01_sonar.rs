use std::env;
use std::fs;

fn count_increases(input: &[i64]) -> i64 {
    let mut increases = 0;
    for i in 1..input.len() {
        if input[i] > input[i - 1] {
            increases += 1;
        }
    }
    increases
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let input = fs::read_to_string(&args[1])
        .expect("couldn't read file")
        .trim()
        .split("\n")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    println!("{}", count_increases(&input));
}
