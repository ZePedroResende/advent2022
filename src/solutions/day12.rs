use crate::{load_file, parse_lines, AoCDay};
use std::collections::HashMap;
use std::collections::HashSet;
pub struct Code;

fn traversal(
    graph: HashMap<String, Vec<String>>,
    node: String,
    visited: &mut HashSet<String>,
) -> usize {
    if node == "end" {
        return 1;
    }
    if node.chars().all(|c| c.is_lowercase()) && visited.contains(&node) {
        return 0;
    }

    visited.insert(node.clone());

    let out = graph
        .get(&(node.clone()))
        .unwrap()
        .iter()
        .map(|n| traversal(graph.clone(), n.into(), visited))
        .sum();

    visited.remove(&node);

    out
}

fn traversal_p2<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    node: &'a str,
    visited: &mut Vec<&'a str>,
    mut twice: bool,
) -> usize {
    if node == "end" {
        return 1;
    }
    if node.chars().all(|c| c.is_lowercase()) && visited.contains(&node) {
        if twice || node == "start" {
            return 0;
        }
        twice = true;
    }
    visited.push(node);
    let out = graph[node]
        .iter()
        .map(|n| traversal_p2(graph, n, visited, twice))
        .sum();
    visited.pop();
    out
}

impl AoCDay for Code {
    fn part1(&self, _input: &mut dyn std::io::Read, _extra_argss: &[String]) -> String {
        let data = load_file(_input);
        let lines: Vec<String> = parse_lines::<String>(&data).collect();
        let graph: HashMap<String, Vec<String>> =
            lines.iter().fold(HashMap::new(), |mut acc, line| {
                let (a, b) = line.split_once("-").unwrap();
                acc.entry(a.to_string())
                    .or_insert_with(Vec::new)
                    .push(b.to_string());
                acc.entry(b.to_string())
                    .or_insert_with(Vec::new)
                    .push(a.to_string());

                acc
            });

        let number = traversal(graph, "start".to_string(), &mut HashSet::new());
        number.to_string()
    }
    fn part2(&self, _input: &mut dyn std::io::Read, _extra_args: &[String]) -> String {
        let data = load_file(_input);
        let lines: Vec<String> = parse_lines::<String>(&data).collect();
        let graph: HashMap<&str, Vec<&str>> = lines.iter().fold(HashMap::new(), |mut acc, line| {
            let (a, b) = line.split_once("-").unwrap();
            acc.entry(a).or_insert_with(Vec::new).push(b);
            acc.entry(b).or_insert_with(Vec::new).push(a);

            acc
        });

        //can't use the hashset because of duplicated nodes on path and we want to pop only the
        //node that was last added and no both of the duplicated
        let number = traversal_p2(&graph, "start", &mut Vec::new(), false);
        number.to_string()
    }
}
