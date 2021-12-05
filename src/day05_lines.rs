use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs;

fn count_overlaps(input: &[Vec<(i64, i64)>], diag: bool) -> usize {
    let mut counts = HashMap::new();
    for line in input.iter() {
        if !diag && (line[0].0 != line[1].0 && line[0].1 != line[1].1) {
            continue;
        }
        let mut i = line[0].0;
        let mut j = line[0].1;
        while !(i == line[1].0 && j == line[1].1) {
            *counts.entry((i, j)).or_insert(0) += 1;
            i += match i.cmp(&line[1].0) {
                Ordering::Greater => -1,
                Ordering::Less => 1,
                Ordering::Equal => 0,
            };
            j += match j.cmp(&line[1].1) {
                Ordering::Greater => -1,
                Ordering::Less => 1,
                Ordering::Equal => 0,
            };
        }
        // last one, since end is inclusive
        *counts.entry((i, j)).or_insert(0) += 1;
    }

    counts.values().filter(|&count| *count > 1).count()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let input_str = fs::read_to_string(&args[1]).expect("couldn't read file");
    let input = input_str
        .trim()
        .split('\n')
        .map(|s| {
            let parts = s.split("->").map(|s1| s1.trim());
            let mut out = Vec::new();
            for part in parts {
                let start_end = part
                    .split(',')
                    .map(|s1| s1.parse::<i64>().unwrap())
                    .collect::<Vec<_>>();
                out.push((start_end[0], start_end[1]));
            }
            out
        })
        .collect::<Vec<_>>();
    println!("{:?}", count_overlaps(&input, false));
    println!("{:?}", count_overlaps(&input, true));
}
