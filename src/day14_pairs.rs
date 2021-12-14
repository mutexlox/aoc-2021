use std::collections::HashMap;
use std::env;
use std::fs;

fn run_rules(start: &str, rules: &HashMap<String, char>, iters: usize) -> i64 {
    let mut freqs = HashMap::new();
    for c in start.chars() {
        *freqs.entry(c).or_insert(0) += 1;
    }
    let mut pairs = HashMap::new();
    for i in 0..start.len() - 1 {
        *pairs
            .entry(start.get(i..i + 2).unwrap().to_string())
            .or_insert(0) += 1;
    }
    for _ in 0..iters {
        let mut pairs_new = pairs.clone();
        for (p, count) in pairs.iter() {
            if let Some(c) = rules.get(p) {
                let chars = p.chars().collect::<Vec<_>>();
                *pairs_new.entry(p.to_string()).or_insert(0) -= count;
                *pairs_new.entry(format!("{}{}", chars[0], c)).or_insert(0) += count;
                *pairs_new.entry(format!("{}{}", c, chars[1])).or_insert(0) += count;
                *freqs.entry(*c).or_insert(0) += count;
            }
        }
        pairs = pairs_new;
    }
    let max = freqs.iter().max_by_key(|(_, v)| *v).unwrap();
    let min = freqs.iter().min_by_key(|(_, v)| *v).unwrap();
    max.1 - min.1
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let input_str = fs::read_to_string(&args[1]).expect("couldn't read file");
    let input = input_str.trim().split('\n').collect::<Vec<_>>();
    let start = input[0];
    let rule_slice = &input[2..];
    let mut rules = HashMap::new();
    for rule in rule_slice.iter() {
        let split = rule.split(" -> ").collect::<Vec<_>>();
        rules.insert(split[0].to_string(), split[1].chars().next().unwrap());
    }
    println!("{}", run_rules(start, &rules, 10));
    println!("{}", run_rules(start, &rules, 40));
}
