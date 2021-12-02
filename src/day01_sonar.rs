use std::env;
use std::fs;

fn count_increases(input: &[i64]) -> usize {
    input
        .iter()
        .enumerate()
        .filter(|(i, &item)| *i > 0 && item > input[*i - 1])
        .count()
}

fn count_increases_window(input: &[i64], win_size: usize) -> i64 {
    let mut increases = 0;
    let mut window: i64 = input.iter().take(win_size).sum();
    for i in win_size..input.len() {
        let new_win = window - input[i - win_size] + input[i];
        if new_win > window {
            increases += 1;
        }
        window = new_win;
    }
    increases
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let input = fs::read_to_string(&args[1])
        .expect("couldn't read file")
        .trim()
        .split('\n')
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    println!("{}", count_increases(&input));
    println!("{}", count_increases_window(&input, 3));
}
