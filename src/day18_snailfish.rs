use crate::SnailfishNumParseErr::SnailfishError;
use std::env;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug, PartialOrd, PartialEq, Clone)]
enum PairElem {
    Pair(Box<SnailfishNum>),
    Lit(i64),
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
struct SnailfishNum {
    left: PairElem,
    right: PairElem,
}

#[derive(Debug, PartialEq, Clone)]
enum SnailfishNumParseErr {
    ParseIntErr(ParseIntError),
    SnailfishError(String),
}

fn find_matching_brace(s: &str) -> Option<usize> {
    if !s.starts_with('[') {
        return None;
    }
    let mut count = 0;
    for (i, c) in s.chars().enumerate() {
        if c == '[' {
            count += 1;
        } else if c == ']' {
            count -= 1;
            if count == 0 {
                return Some(i);
            }
        }
    }
    None
}

impl FromStr for SnailfishNum {
    type Err = SnailfishNumParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with('[') || !s.ends_with(']') {
            return Err(SnailfishError("not wrapped in [ ]".to_string()));
        }

        let rest = s.chars().skip(1).take(s.len() - 2).collect::<String>();
        let i;
        let left = if rest.starts_with('[') {
            let matching = find_matching_brace(&rest).ok_or_else(
                || SnailfishNumParseErr::SnailfishError("No matching ] for [".to_string()),
            )?;
            i = matching + 2; // skip ] and ,
            PairElem::Pair(Box::new(
                rest.chars()
                    .take(matching + 1)
                    .collect::<String>()
                    .parse::<SnailfishNum>()?,
            ))
        } else {
            i = rest.find(',').ok_or_else(|| SnailfishNumParseErr::SnailfishError(
                "Missing ,".to_string(),
            ))? + 1;
            PairElem::Lit(
                rest.chars()
                    .take_while(|c| *c != ',')
                    .collect::<String>()
                    .parse::<i64>()
                    .map_err(SnailfishNumParseErr::ParseIntErr)?,
            )
        };
        let right_str = rest.chars().skip(i).collect::<String>();
        let right = if right_str.starts_with('[') {
            let matching = find_matching_brace(&right_str).ok_or_else(||
                SnailfishNumParseErr::SnailfishError("No matching ] for [".to_string()),
            )?;
            PairElem::Pair(Box::new(
                right_str
                    .chars()
                    .take(matching + 1)
                    .collect::<String>()
                    .parse::<SnailfishNum>()?,
            ))
        } else {
            PairElem::Lit(
                right_str
                    .chars()
                    .take_while(|c| *c != ']')
                    .collect::<String>()
                    .parse::<i64>()
                    .map_err(SnailfishNumParseErr::ParseIntErr)?,
            )
        };
        Ok(SnailfishNum { left, right })
    }
}

impl ToString for SnailfishNum {
    fn to_string(&self) -> String {
        let mut out = String::new();
        out.push('[');
        out.push_str(&match &self.left {
            PairElem::Lit(x) => x.to_string(),
            PairElem::Pair(p) => p.to_string(),
        });
        out.push(',');
        out.push_str(&match &self.right {
            PairElem::Lit(x) => x.to_string(),
            PairElem::Pair(p) => p.to_string(),
        });
        out.push(']');

        out
    }
}

impl SnailfishNum {
    fn explode_first(&self) -> (SnailfishNum, bool) {
        // gross.
        let as_chars = self.to_string().chars().collect::<Vec<_>>();
        let mut depth = 0;
        let mut i = 0;
        let mut out = String::new();
        let mut exploded = false;
        let mut left_start = None;
        let mut left_to_add = None;
        let mut next_num_add = None;
        while i < as_chars.len() {
            if as_chars[i] == '[' {
                depth += 1;
            } else if as_chars[i] == ']' {
                depth -= 1;
            } else if let Some(x) = next_num_add {
                if as_chars[i].is_ascii_digit() {
                    let num_str = as_chars
                        .iter()
                        .skip(i)
                        .take_while(|&c| *c != ',' && *c != ']')
                        .collect::<String>();
                    let mut num = num_str.parse::<i64>().unwrap();
                    num += x;
                    out.push_str(&num.to_string());
                    i += num_str.len();
                    next_num_add = None;
                }
            }
            if depth > 4 && !exploded {
                let left = as_chars
                    .iter()
                    .skip(i + 1)
                    .take_while(|&c| *c != ',')
                    .collect::<String>()
                    .parse::<i64>()
                    .unwrap();
                left_to_add = Some(left);
                let mut j = i;
                while j > 0 && !as_chars[j].is_ascii_digit() {
                    j -= 1;
                }
                if j > 0 {
                    while as_chars[j].is_ascii_digit() {
                        j -= 1;
                    }
                    j += 1;
                    // Deal with this after the loop
                    left_start = Some(j);
                }
                let right = as_chars
                    .iter()
                    .skip(i)
                    .skip_while(|&c| *c != ',')
                    .skip(1)
                    .take_while(|&c| *c != ']')
                    .collect::<String>()
                    .parse::<i64>()
                    .unwrap();
                next_num_add = Some(right);
                out.push('0');
                exploded = true;
                // skip to just past this
                let next_close = i + as_chars.iter().skip(i).take_while(|&c| *c != ']').count();
                i = next_close;
            } else {
                out.push(as_chars[i]);
            }
            i += 1;
        }
        if let Some(j) = left_start {
            let num_str = as_chars
                .iter()
                .skip(j)
                .take_while(|&c| *c != ',' && *c != ']')
                .collect::<String>();
            let mut num = num_str.parse::<i64>().unwrap();
            num += left_to_add.unwrap();
            out.replace_range(j..j + num_str.len(), &num.to_string());
        }
        let ret = out.parse::<SnailfishNum>().unwrap();
        (ret, exploded)
    }

    fn split_first(&self, mut already_split: bool) -> (SnailfishNum, bool) {
        if already_split {
            return (self.clone(), true);
        }
        let left = match &self.left {
            PairElem::Pair(num) => {
                let (new, split) = num.split_first(already_split);
                if split {
                    already_split = true;
                }
                PairElem::Pair(Box::new(new))
            }
            PairElem::Lit(x) => {
                if *x >= 10 {
                    already_split = true;
                    PairElem::Pair(Box::new(SnailfishNum {
                        left: PairElem::Lit(x / 2),
                        right: PairElem::Lit((x - 1) / 2 + 1),
                    }))
                } else {
                    PairElem::Lit(*x)
                }
            }
        };
        if already_split {
            return (
                SnailfishNum {
                    left,
                    right: self.right.clone(),
                },
                true,
            );
        }
        let right = match &self.right {
            PairElem::Pair(num) => {
                let (new, split) = num.split_first(already_split);
                if split {
                    already_split = true;
                }
                PairElem::Pair(Box::new(new))
            }
            PairElem::Lit(x) => {
                if *x >= 10 {
                    already_split = true;
                    PairElem::Pair(Box::new(SnailfishNum {
                        left: PairElem::Lit(x / 2),
                        right: PairElem::Lit((x - 1) / 2 + 1),
                    }))
                } else {
                    PairElem::Lit(*x)
                }
            }
        };
        (SnailfishNum { left, right }, already_split)
    }

    fn reduce(&self) -> SnailfishNum {
        // first, try to explode
        let (mut cur, mut exploded) = self.explode_first();
        let mut split = false;
        if !exploded {
            // workaround until destructuring assignments are stable
            let tmp = cur.split_first(false);
            cur = tmp.0;
            split = tmp.1;
        }
        while exploded || split {
            let tmp = cur.explode_first();
            cur = tmp.0;
            exploded = tmp.1;
            if exploded {
                continue;
            }
            let tmp = cur.split_first(false);
            cur = tmp.0;
            split = tmp.1;
        }
        cur
    }

    fn add(&self, other: &SnailfishNum) -> SnailfishNum {
        let added = SnailfishNum {
            left: PairElem::Pair(Box::new(self.clone())),
            right: PairElem::Pair(Box::new(other.clone())),
        };
        added.reduce()
    }

    fn magnitude(&self) -> i64 {
        let left = 3 * match &self.left {
            PairElem::Lit(x) => *x,
            PairElem::Pair(p) => p.magnitude(),
        };
        let right = 2 * match &self.right {
            PairElem::Lit(x) => *x,
            PairElem::Pair(p) => p.magnitude(),
        };
        left + right
    }
}

fn add_all(nums: &[SnailfishNum]) -> SnailfishNum {
    let mut out = nums[0].clone();
    for n in nums.iter().skip(1) {
        out = out.add(n);
    }
    out
}

fn largest_pair_magnitude(nums: &[SnailfishNum]) -> i64 {
    let mut max = i64::MIN;
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if i == j {
                continue;
            }
            let mag = nums[i].add(&nums[j]).magnitude();
            if mag > max {
                max = mag;
            }
        }
    }
    max
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let input_str = fs::read_to_string(&args[1]).expect("couldn't read file");
    let input = input_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<SnailfishNum>().unwrap())
        .collect::<Vec<_>>();
    println!("{}", add_all(&input).magnitude());
    println!("{}", largest_pair_magnitude(&input));
}
