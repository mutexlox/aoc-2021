use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

fn count_num_uniques(input: &[Vec<Vec<&str>>]) -> usize {
    input
        .iter()
        .map(|v| v[1].clone())
        // filter to include only 1, 4, 7, 8.
        .map(|v| {
            v.iter()
                .filter(|s| s.len() == 2 || s.len() == 4 || s.len() == 3 || s.len() == 7)
                .count()
        })
        .sum()
}

fn solve(input: &[Vec<Vec<&str>>]) -> i64 {
    let mut nums = Vec::new();
    let mut segment_map = HashMap::new();
    segment_map.insert("abcefg", 0);
    segment_map.insert("cf", 1);
    segment_map.insert("acdeg", 2);
    segment_map.insert("acdfg", 3);
    segment_map.insert("bcdf", 4);
    segment_map.insert("abdfg", 5);
    segment_map.insert("abdefg", 6);
    segment_map.insert("acf", 7);
    segment_map.insert("abcdefg", 8);
    segment_map.insert("abcdfg", 9);

    // wires might be different for each entry, so solve each separately
    for entry in input.iter() {
        // map wire to output segment
        let mut mapping = HashMap::new();
        for c in "abcdefg".chars() {
            mapping.insert(c, "abcdefg".chars().collect::<HashSet<_>>());
        }
        let mut one = HashSet::new();
        let mut four = HashSet::new();
        let mut seven = HashSet::new();
        let mut sixes = "abcdefg".chars().collect::<HashSet<_>>();
        // Find known ones
        for item in entry[0].iter() {
            if item.len() == 2 {
                one = item.chars().collect();
            } else if item.len() == 4 {
                four = item.chars().collect();
            } else if item.len() == 3 {
                seven = item.chars().collect();
            } else if item.len() == 6 {
                sixes = sixes
                    .intersection(&item.chars().collect())
                    .copied()
                    .collect();
            }
        }
        // a is 7 \ 1
        let a_char = seven.difference(&one).next().unwrap();
        mapping.get_mut(a_char).unwrap().retain(|c| 'a' == *c);
        for c in "abcdefg".chars() {
            if c != *a_char {
                mapping.get_mut(&c).unwrap().remove(&'a'); // cannot map to 'a'
            }
        }
        let c_f_chars = one.clone();
        for c in c_f_chars.iter() {
            mapping.get_mut(c).unwrap().retain(|c| "cf".contains(*c));
        }
        // remove C, F from all others
        for c in "abcdefg".chars() {
            if !c_f_chars.contains(&c) {
                mapping.get_mut(&c).unwrap().remove(&'c');
                mapping.get_mut(&c).unwrap().remove(&'f');
            }
        }
        // 4 \ 1 gives  {B, D}
        let b_d_chars = four.difference(&one).copied().collect::<Vec<_>>();
        for c in b_d_chars.iter() {
            mapping.get_mut(c).unwrap().retain(|c| "bd".contains(*c));
        }
        // remove B,D from all others
        for c in "abcdefg".chars() {
            if !b_d_chars.contains(&c) {
                mapping.get_mut(&c).unwrap().remove(&'b');
                mapping.get_mut(&c).unwrap().remove(&'d');
            }
        }

        // those that are *not* in sixes are c, d, e
        let mut cde_chars = HashSet::new();
        for c in "abcdefg".chars() {
            if !sixes.contains(&c) {
                cde_chars.insert(c);
            }
        }
        for c in cde_chars.iter() {
            mapping.get_mut(c).unwrap().retain(|c| "cde".contains(*c));
        }
        for c in "abcdefg".chars() {
            if !cde_chars.contains(&c) {
                mapping.get_mut(&c).unwrap().remove(&'c');
                mapping.get_mut(&c).unwrap().remove(&'d');
                mapping.get_mut(&c).unwrap().remove(&'e');
            }
        }

        let out_nums = entry[1]
            .iter()
            .map(|s| {
                let mut mapped = s
                    .chars()
                    .map(|c| mapping[&c].iter().next().unwrap())
                    .copied()
                    .collect::<Vec<char>>();
                mapped.sort_unstable();
                let mapped_str = mapped.iter().collect::<String>();
                segment_map[&mapped_str as &str]
            })
            .collect::<Vec<_>>();
        nums.push(out_nums[0] * 1000 + out_nums[1] * 100 + out_nums[2] * 10 + out_nums[3]);
    }
    nums.iter().sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let input_str = fs::read_to_string(&args[1]).expect("couldn't read file");
    let input = input_str
        .trim()
        .split('\n')
        .map(|s| {
            s.split('|')
                .map(|s| s.split_whitespace().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    println!("{}", count_num_uniques(&input));
    println!("{}", solve(&input));
}
