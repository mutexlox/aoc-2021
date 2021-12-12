use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

fn is_lowercase(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_lowercase())
}

fn num_paths<'a>(graph: &HashMap<&str, Vec<&'a str>>, allow_dupe_lower: bool) -> usize {
    fn dfs<'a>(
        node: &'a str,
        graph: &HashMap<&str, Vec<&'a str>>,
        path: &mut Vec<&'a str>,
        allow_dupe_lower: bool,
    ) -> usize {
        if is_lowercase(node) && path.contains(&node) {
            if !allow_dupe_lower {
                return 0;
            }
            // part 2 logic
            if node == "start" || node == "end" {
                return 0;
            }
            if path.iter().filter(|&s| *s == node).count() == 2 {
                return 0;
            }
            let lower = path.iter().filter(|s| is_lowercase(s)).collect::<Vec<_>>();
            let set = lower.iter().collect::<HashSet<_>>();
            if lower.len() != set.len() {
                return 0;
            }
        }
        path.push(node);
        let mut paths = 0;
        for neighbor in graph[node].iter() {
            if *neighbor == "end" {
                paths += 1;
            } else {
                paths += dfs(neighbor, graph, path, allow_dupe_lower);
            }
        }
        path.pop();
        paths
    }

    let mut path = Vec::new();
    dfs("start", graph, &mut path, allow_dupe_lower)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let input_str = fs::read_to_string(&args[1]).expect("couldn't read file");
    let input = input_str
        .trim()
        .split('\n')
        .map(|s| s.split('-').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut graph = HashMap::new();
    for edge in input.iter() {
        graph.entry(edge[0]).or_insert_with(Vec::new).push(edge[1]);
        graph.entry(edge[1]).or_insert_with(Vec::new).push(edge[0]);
    }
    println!("{}", num_paths(&graph, false));
    println!("{}", num_paths(&graph, true));
}
