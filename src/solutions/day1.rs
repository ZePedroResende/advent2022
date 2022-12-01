use crate::{load_file, parse_lines, AoCDay};
use std::collections::BinaryHeap;
use std::io::Read;

pub struct Code;
impl AoCDay for Code {
    fn part1(&self, input: &mut dyn Read, _extra_argss: &[String]) -> String {
        let input = load_file(input);
        let calories = input.split("\n\n");
        let sum = calories.map(|s| {
            s.lines()
                .map(|x| x.parse::<usize>().unwrap())
                .sum::<usize>()
        });

        let max = sum.max().unwrap();

        max.to_string()
    }

    fn part2(&self, input: &mut dyn std::io::Read, _extra_args: &[String]) -> String {
        let input = load_file(input);
        let calories = input.split("\n\n");
        let sum = calories.map(|s| {
            s.lines()
                .map(|x| x.parse::<usize>().unwrap())
                .sum::<usize>()
        });

        let mut heap = sum.collect::<BinaryHeap<usize>>();

        let mut top_three = Vec::new();
        for _ in 0..3 {
            if let Some(v) = heap.pop() {
                top_three.push(v);
            }
        }

        top_three.iter().sum::<usize>().to_string()
    }
}
