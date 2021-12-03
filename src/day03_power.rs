use std::env;
use std::fs;

fn count_ones(input: &[String]) -> Vec<usize> {
    // Track number of ones in each position
    let mut one_counts = Vec::new();
    one_counts.resize(input[0].len(), 0);
    input.iter().for_each(|s| {
        for (i, c) in s.char_indices() {
            if c == '1' {
                one_counts[i] += 1;
            }
        }
    });
    one_counts
}

fn compute_power_consumption(input: &[String]) -> i64 {
    let one_counts = count_ones(input);

    let mut gamma = 0;
    let mut epsilon = 0;
    for count in one_counts.iter() {
        gamma *= 2;
        epsilon *= 2;
        if *count > input.len() / 2 {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    gamma * epsilon
}

fn compute_life_support(input: &[String]) -> i64 {
    let mut oxygen_input = input.to_vec();
    let mut i = 0;
    while oxygen_input.len() > 1 && i < oxygen_input[0].len() {
        let one_counts = count_ones(&oxygen_input);
        if one_counts[i] >= (oxygen_input.len() + 1) / 2 {
            oxygen_input.retain(|s| s.get(i..i + 1) == Some("1"));
        } else {
            oxygen_input.retain(|s| s.get(i..i + 1) == Some("0"));
        }
        i += 1;
    }
    assert_eq!(oxygen_input.len(), 1);
    let oxygen = i64::from_str_radix(&oxygen_input[0], 2).unwrap();

    let mut co2_input = input.to_vec();
    i = 0;
    while co2_input.len() > 1 && i < co2_input[0].len() {
        let one_counts = count_ones(&co2_input);
        if one_counts[i] < (co2_input.len() + 1) / 2 {
            co2_input.retain(|s| s.get(i..i + 1) == Some("1"));
        } else {
            co2_input.retain(|s| s.get(i..i + 1) == Some("0"));
        }
        i += 1;
    }
    assert_eq!(co2_input.len(), 1);
    let co2 = i64::from_str_radix(&co2_input[0], 2).unwrap();

    oxygen * co2
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let input = fs::read_to_string(&args[1])
        .expect("couldn't read file")
        .trim()
        .split('\n')
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();
    println!("{}", compute_power_consumption(&input));
    println!("{}", compute_life_support(&input));
}
