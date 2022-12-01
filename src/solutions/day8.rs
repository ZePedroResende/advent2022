use crate::{load_file, parse_lines, AoCDay};
pub struct Code;
use std::collections::HashSet;
use std::str::FromStr;

fn is_part1(str: &str) -> bool {
    str.len() < 5 || str.len() == 7
}

#[derive(Debug, Clone, Copy)]
struct Num {
    number: i32,
}

fn classify(s: &[&str], map: &mut [HashSet<char>; 10]) {
    s.iter().for_each(|l| match l.len() {
        2 => map[1] = l.chars().into_iter().collect(),

        3 => map[7] = l.chars().into_iter().collect(),

        4 => map[4] = l.chars().into_iter().collect(),

        5 => {
            let tmp: HashSet<char> = l.chars().into_iter().collect();

            if map[1].difference(&tmp).count() == 0 {
                map[3] = tmp;
            } else if tmp.difference(&map[4]).count() == 2 {
                map[5] = tmp;
            } else if tmp.difference(&map[4]).count() == 3 {
                map[2] = tmp;
            }
        }
        6 => {
            let tmp: HashSet<char> = l.chars().into_iter().collect();
            if map[1].difference(&tmp).count() == 1 {
                map[6] = tmp;
            } else if map[5].difference(&tmp).count() == 0 {
                map[9] = tmp;
            } else {
                map[0] = tmp;
            }
        }
        7 => map[8] = l.chars().into_iter().collect(),

        _ => (),
    });
}

impl FromStr for Num {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line: Vec<Vec<&str>> = s.split(" | ").map(|v| v.split(' ').collect()).collect();

        let mut map: [HashSet<char>; 10] = Default::default();

        line.iter().for_each(|x| classify(x, &mut map));

        let mut number = 0;

        for (i, pat) in line[1].iter().rev().enumerate() {
            let tmp: HashSet<char> = pat.chars().into_iter().collect();

            let number_map = map.iter().position(|x| x == &tmp).expect("failed") as i32;

            number += number_map * i32::pow(10, i as u32);
        }

        Ok(Self { number })
    }
}

impl AoCDay for Code {
    fn part1(&self, _input: &mut dyn std::io::Read, _extra_argss: &[String]) -> String {
        let data = load_file(_input);
        let numbers = data
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(|x| {
                let split: Vec<Vec<&str>> = x
                    .split(" | ")
                    .map(|e| e.split(' ').collect::<Vec<&str>>())
                    .collect();

                split[1].iter().filter(|s| is_part1(s)).count() as u64
            })
            .collect::<Vec<u64>>()
            .iter()
            .sum::<u64>()
            .to_string();

        numbers
        //let lines: Vec<Line> = parse_lines::<Line>(&data).collect();
    }
    fn part2(&self, _input: &mut dyn std::io::Read, _extra_args: &[String]) -> String {
        let data = load_file(_input);
        let nums: Vec<Num> = parse_lines::<Num>(&data).collect();
        nums.iter().map(|x| x.number).sum::<i32>().to_string()
    }
}
