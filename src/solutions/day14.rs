use crate::{load_file, AoCDay};
use std::collections::HashMap;
pub struct Code;

fn calculate(rules_str: &str, template: &str, iteration: usize) -> u64 {
    let rules: HashMap<(char, char), char> = rules_str.lines().fold(
        HashMap::new(),
        |mut acc: HashMap<(char, char), char>, l: &str| {
            let (element_a, element_b) = l.rsplit_once(" -> ").unwrap();
            let mut chars = element_a.chars();

            acc.insert(
                (chars.next().unwrap(), chars.next().unwrap()),
                element_b.chars().next().unwrap(),
            );

            acc
        },
    );

    let pairs: Vec<(char, char)> = template
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .map(|c| (c[0], c[1]))
        .collect();

    let mut counter_chars: HashMap<char, u64> = HashMap::new();
    let mut counter_pairs: HashMap<(char, char), u64> = HashMap::new();

    for pair in pairs.iter() {
        *counter_chars.entry(pair.0).or_insert(0) += 1_u64;
        *counter_chars.entry(pair.1).or_insert(0) += 1_u64;

        *counter_pairs.entry(*pair).or_insert(0) += 1_u64;
    }

    for _ in 0..iteration {
        for (pair, times) in counter_pairs.clone().iter() {
            let value: &char = rules.get(pair).unwrap();

            *counter_pairs.entry((pair.0, *value)).or_insert(0) += *times as u64;
            *counter_pairs.entry((*value, pair.1)).or_insert(0) += *times as u64;
            *counter_pairs.entry(*pair).or_insert(0) -= *times as u64;

            *counter_chars.entry(*value).or_insert(0) += *times as u64;
        }
    }

    let values = counter_chars.values();
    let max = values.clone().max().unwrap();
    let min = values.min().unwrap();

    *max - *min
}

impl AoCDay for Code {
    fn part1(&self, _input: &mut dyn std::io::Read, _extra_argss: &[String]) -> String {
        let data = load_file(_input);
        let (template, rules_str) = data.split_once("\n\n").unwrap();

        calculate(rules_str, template, 10).to_string()
    }

    fn part2(&self, _input: &mut dyn std::io::Read, _extra_args: &[String]) -> String {
        let data = load_file(_input);
        let (template, rules_str) = data.split_once("\n\n").unwrap();

        calculate(rules_str, template, 40).to_string()
    }
}
