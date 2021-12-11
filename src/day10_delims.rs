use std::env;
use std::fs;

fn check_corrupted_line(line: &str) -> Option<u64> {
    let mut paren_count = 0;
    let mut brace_count = 0;
    let mut squiggle_count = 0;
    let mut angle_count = 0;
    let mut most_recent_opens = Vec::new();
    for c in line.chars() {
        if "([{<".contains(c) {
            most_recent_opens.push(c);
        }
        match c {
            '(' => paren_count += 1,
            '[' => brace_count += 1,
            '{' => squiggle_count += 1,
            '<' => angle_count += 1,
            ')' => {
                paren_count -= 1;
                if *most_recent_opens.last().unwrap() != '(' || paren_count < 0 {
                    return Some(3);
                }
                most_recent_opens.pop();
            }
            ']' => {
                brace_count -= 1;
                if *most_recent_opens.last().unwrap() != '[' || brace_count < 0 {
                    return Some(57);
                }
                most_recent_opens.pop();
            }
            '}' => {
                squiggle_count -= 1;
                if *most_recent_opens.last().unwrap() != '{' || squiggle_count < 0 {
                    return Some(1197);
                }
                most_recent_opens.pop();
            }
            '>' => {
                angle_count -= 1;
                if *most_recent_opens.last().unwrap() != '<' || angle_count < 0 {
                    return Some(25137);
                }
                most_recent_opens.pop();
            }
            _ => panic!("invalid char"),
        };
    }
    None
}

fn count_wrong_character(input: &[&str]) -> u64 {
    let mut invalid_score = 0;
    for line in input.iter() {
        if let Some(x) = check_corrupted_line(line) {
            invalid_score += x;
        }
    }
    invalid_score
}

fn check_incomplete_line(line: &str) -> Option<u64> {
    let mut most_recent_opens = Vec::new();
    for c in line.chars() {
        if "([{<".contains(c) {
            most_recent_opens.push(c);
        } else {
            most_recent_opens.pop();
        }
    }
    if most_recent_opens.is_empty() {
        return None;
    }
    let to_fix = most_recent_opens.iter().rev().map(|c| match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("invalid char {}", c),
    });
    let mut score = 0;
    for c in to_fix {
        score *= 5;
        score += match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("invalid char {}", c),
        }
    }
    Some(score)
}

fn score_incompletes(input: &[&str]) -> u64 {
    let mut scores = Vec::new();
    for line in input.iter() {
        if check_corrupted_line(line).is_some() {
            continue;
        }
        // otherwise, not corrupt but may be incomplete
        if let Some(score) = check_incomplete_line(line) {
            scores.push(score);
        }
    }
    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let input_str = fs::read_to_string(&args[1]).expect("couldn't read file");
    let input = input_str.trim().split('\n').collect::<Vec<_>>();
    println!("{}", count_wrong_character(&input));
    println!("{}", score_incompletes(&input));
}
