use std::env;
use std::fs;

fn neighbors(i: usize, j: usize, max: usize) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    if i > 0 {
        out.push((i - 1, j));
        if j > 0 {
            out.push((i - 1, j - 1));
        }
        if j < max - 1 {
            out.push((i - 1, j + 1));
        }
    }
    if j > 0 {
        out.push((i, j - 1));
    }
    if j < max - 1 {
        out.push((i, j + 1));
    }
    if i < max - 1 {
        out.push((i + 1, j));
        if j > 0 {
            out.push((i + 1, j - 1));
        }
        if j < max - 1 {
            out.push((i + 1, j + 1));
        }
    }

    out
}

fn flash_neighbors(i: usize, j: usize, out: &mut [Vec<u32>]) -> usize {
    let mut flashes = 0;
    for &(new_i, new_j) in neighbors(i, j, out.len()).iter() {
        if out[new_i][new_j] == u32::MAX {
            continue;
        }
        out[new_i][new_j] += 1;
        if out[new_i][new_j] > 9 {
            out[new_i][new_j] = u32::MAX; // so it doesn't flash again
            flashes += 1 + flash_neighbors(new_i, new_j, out);
        }
    }
    flashes
}

fn step(input: &[Vec<u32>]) -> (Vec<Vec<u32>>, usize) {
    let mut flashes = 0;
    let mut out = input
        .iter()
        .map(|r| r.iter().map(|i| i + 1).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for i in 0..out.len() {
        for j in 0..out[0].len() {
            if out[i][j] > 9 && out[i][j] != u32::MAX {
                out[i][j] = u32::MAX;
                flashes += 1 + flash_neighbors(i, j, &mut out);
            }
        }
    }
    out.iter_mut().for_each(|r| {
        r.iter_mut().for_each(|x| {
            if *x == u32::MAX {
                *x = 0
            }
        })
    });

    (out, flashes)
}

fn flashes_after(input: &[Vec<u32>], count: u32) -> usize {
    let mut flashes = 0;
    let mut next = input.to_vec();
    for _ in 0..count {
        let (new_next, new_flashes) = step(&next);
        next = new_next;
        flashes += new_flashes;
    }

    flashes
}

fn go_until_all_flash(input: &[Vec<u32>]) -> usize {
    let mut i = 0;
    let mut next = input.to_vec();
    let mut flashes = 0;
    while flashes != input.len() * input[0].len() {
        let (new_next, new_flashes) = step(&next);
        next = new_next;
        flashes = new_flashes;
        i += 1;
    }

    i
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
    println!("{:?}", flashes_after(&input, 100));
    println!("{:?}", go_until_all_flash(&input));
}
