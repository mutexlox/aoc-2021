use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Fold {
    Horiz(i64),
    Vert(i64),
}

fn apply_fold(points: &mut HashSet<(i64, i64)>, fold: Fold) {
    let mut to_remove = Vec::new();
    let mut to_add = Vec::new();
    match fold {
        Fold::Horiz(y_line) => {
            for (x, y) in points.iter() {
                if *y <= y_line {
                    continue;
                }
                to_remove.push((*x, *y));
                // y = y_line + 1 -> insert y - 2
                // y = y_line + 2 -> insert y - 4
                // y = y_line + 3 -> insert y - 6
                // insert y - (y - y_line) * 2
                to_add.push((*x, y - (y - y_line) * 2));
            }
        }
        Fold::Vert(x_line) => {
            for (x, y) in points.iter() {
                if *x <= x_line {
                    continue;
                }
                to_remove.push((*x, *y));
                to_add.push((x - (x - x_line) * 2, *y));
            }
        }
    }
    for p in to_remove.iter() {
        points.remove(p);
    }
    for p in to_add.iter() {
        points.insert(*p);
    }
}

fn count_distinct_points_after(points: &HashSet<(i64, i64)>, folds: &[Fold]) -> usize {
    let mut scratch = points.clone();
    for &fold in folds.iter() {
        apply_fold(&mut scratch, fold);
    }
    scratch.len()
}

fn apply_folds_and_print(points: &HashSet<(i64, i64)>, folds: &[Fold]) {
    let mut scratch = points.clone();
    for &fold in folds.iter() {
        apply_fold(&mut scratch, fold);
    }
    let mut min_x = i64::MAX;
    let mut min_y = i64::MAX;
    let mut max_x = i64::MIN;
    let mut max_y = i64::MIN;
    for (x, y) in scratch.iter() {
        if *x < min_x {
            min_x = *x;
        }
        if *x > max_x {
            max_x = *x;
        }
        if *y < min_y {
            min_y = *y;
        }
        if *y > max_y {
            max_y = *y;
        }
    }
    for y in min_y..max_y + 1 {
        for x in min_x..max_x + 1 {
            if scratch.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let input_str = fs::read_to_string(&args[1]).expect("couldn't read file");
    let input = input_str.trim().split('\n').collect::<Vec<_>>();
    let mut points = HashSet::new();
    let mut done_points = false;
    let mut folds = Vec::new();
    for line in input.iter() {
        if line.is_empty() {
            done_points = true;
            continue;
        }
        if done_points {
            let fold = line.split_whitespace().collect::<Vec<_>>()[2]
                .split('=')
                .collect::<Vec<_>>();
            let num = fold[1].parse::<i64>().unwrap();
            if fold[0] == "x" {
                folds.push(Fold::Vert(num));
            } else {
                folds.push(Fold::Horiz(num));
            }
            continue;
        }

        let xy = line
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        points.insert((xy[0], xy[1]));
    }

    println!("{}", count_distinct_points_after(&points, &folds[..1]));
    apply_folds_and_print(&points, &folds);
}
