use std::collections::{HashSet, VecDeque};
use std::env;
use std::fs;

fn risk_levels_low_pts(input: &[Vec<u32>]) -> u32 {
    let mut tot = 0;
    for (i, row) in input.iter().enumerate() {
        for (j, elem) in row.iter().enumerate() {
            if i > 0 && *elem >= input[i - 1][j] {
                continue;
            }
            if i < input.len() - 1 && *elem >= input[i + 1][j] {
                continue;
            }
            if j > 0 && *elem >= input[i][j - 1] {
                continue;
            }
            if j < input[0].len() - 1 && *elem >= input[i][j + 1] {
                continue;
            }
            tot += elem + 1;
        }
    }

    tot
}

fn bfs(i: usize, j: usize, input: &[Vec<u32>], visited: &mut HashSet<(usize, usize)>) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((i, j));
    let mut size = 0;
    while let Some(next) = queue.pop_front() {
        let i = next.0;
        let j = next.1;
        if visited.contains(&(i, j)) {
            continue;
        }
        visited.insert((i, j));
        if input[i][j] == 9 {
            continue;
        }
        size += 1;
        if i > 0 {
            queue.push_back((i - 1, j));
        }
        if i < input.len() - 1 {
            queue.push_back((i + 1, j));
        }
        if j > 0 {
            queue.push_back((i, j - 1));
        }
        if j < input[0].len() - 1 {
            queue.push_back((i, j + 1));
        }
    }

    size
}

fn basins(input: &[Vec<u32>]) -> usize {
    let mut visited = HashSet::new();
    let mut sizes = Vec::new();
    while visited.len() != input.len() * input[0].len() {
        // find first (i, j) not visited
        let mut i = 0;
        let mut j = 0;
        'outer: while i < input.len() {
            while j < input[0].len() {
                if !visited.contains(&(i, j)) {
                    if input[i][j] == 9 {
                        visited.insert((i, j));
                    } else {
                        break 'outer;
                    }
                }
                j += 1;
            }
            j = 0;
            i += 1;
        }
        if i == input.len() {
            assert_eq!(visited.len(), input.len() * input[0].len());
            break;
        }
        sizes.push(bfs(i, j, input, &mut visited));
    }

    sizes.sort_unstable();
    sizes.iter().rev().take(3).product()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let input_str = fs::read_to_string(&args[1]).expect("couldn't read file");
    let input = input_str
        .trim()
        .split('\n')
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    println!("{}", risk_levels_low_pts(&input));
    println!("{}", basins(&input));
}
