use std::collections::HashSet;
use std::env;
use std::fs;

fn board_won(marked: &HashSet<(usize, usize)>) -> bool {
    // check rows
    for row in 0..5 {
        let mut win = true;
        for col in 0..5 {
            if !marked.contains(&(row, col)) {
                win = false;
                break;
            }
        }
        if win {
            return true;
        }
    }
    // check cols
    for col in 0..5 {
        let mut win = true;
        for row in 0..5 {
            if !marked.contains(&(row, col)) {
                win = false;
                break;
            }
        }
        if win {
            return true;
        }
    }
    false
}

fn sum_unmarked(marked: &HashSet<(usize, usize)>, board: &[Vec<i64>]) -> i64 {
    let mut sum = 0;
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if !marked.contains(&(i, j)) {
                sum += board[i][j];
            }
        }
    }

    sum
}

fn winning_score(numbers: &[i64], boards: &[Vec<Vec<i64>>], first: bool) -> i64 {
    let mut marked = Vec::new();
    for _ in 0..boards.len() {
        marked.push(HashSet::new());
    }

    let mut winners = HashSet::new();
    for number in numbers.iter() {
        for (b_idx, board) in boards.iter().enumerate() {
            for (i, row) in board.iter().enumerate() {
                for (j, val) in row.iter().enumerate() {
                    if val == number {
                        marked[b_idx].insert((i, j));
                        // Check if this board won
                        if board_won(&marked[b_idx]) {
                            winners.insert(b_idx);
                            if first || winners.len() == boards.len() {
                                return *number * sum_unmarked(&marked[b_idx], board);
                            }
                        }
                    }
                }
            }
        }
    }

    -1
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let input_str = fs::read_to_string(&args[1]).expect("couldn't read file");
    let input = input_str.trim().split('\n').collect::<Vec<_>>();
    let numbers = input[0]
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut boards = Vec::new();
    let mut cur_board = Vec::new();
    for row in input.iter().skip(2) {
        if row.is_empty() {
            boards.push(cur_board);
            cur_board = Vec::new();
            continue;
        }
        cur_board.push(
            row.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>(),
        );
    }
    boards.push(cur_board);
    println!("{}", winning_score(&numbers, &boards, true));
    println!("{}", winning_score(&numbers, &boards, false));
}
