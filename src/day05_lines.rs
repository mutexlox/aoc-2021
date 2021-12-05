use std::cmp::{max, min};
use std::collections::HashMap;
use std::env;
use std::fs;

fn count_overlaps(input: &[Vec<(i64, i64)>], diag: bool) -> usize {
    let mut counts = HashMap::new();
    // For non-diagonal lines...
    for line in input
        .iter()
        .filter(|l| l[0].0 == l[1].0 || l[0].1 == l[1].1)
    {
        let low_i = min(line[0].0, line[1].0);
        let high_i = max(line[0].0, line[1].0);
        let low_j = min(line[0].1, line[1].1);
        let high_j = max(line[0].1, line[1].1);
        for i in low_i..high_i + 1 {
            for j in low_j..high_j + 1 {
                let counter = counts.entry((i, j)).or_insert(0);
                *counter += 1;
            }
        }
    }
    if diag {
        for line in input
            .iter()
            .filter(|l| l[0].0 != l[1].0 && l[0].1 != l[1].1)
        {
            let mut i = line[0].0;
            let mut j = line[0].1;
            while i != line[1].0 && j != line[1].1 {
                let counter = counts.entry((i, j)).or_insert(0);
                *counter += 1;
                if i > line[1].0 {
                    i -= 1;
                } else {
                    i += 1;
                }
                if j > line[1].1 {
                    j -= 1;
                } else {
                    j += 1;
                }
            }
            let counter = counts.entry((i, j)).or_insert(0);
            *counter += 1;
        }
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
