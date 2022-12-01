use crate::{load_file, parse_lines, AoCDay};
pub struct Code;

impl AoCDay for Code {
    fn part1(&self, _input: &mut dyn std::io::Read, _extra_argss: &[String]) -> String {
        let data = load_file(_input);
        let lines: Vec<String> = parse_lines::<String>(&data).collect();

        lines
            .iter()
            .map(|str| {
                let mut vec = Vec::new();
                str.chars().fold(0, |mut acc, character: char| {
                    match character {
                        ']' => {
                            if vec.pop().unwrap() != '[' {
                                acc += 57;
                            }
                        }
                        ')' => {
                            if vec.pop().unwrap() != '(' {
                                acc += 3;
                            }
                        }
                        '>' => {
                            if vec.pop().unwrap() != '<' {
                                acc += 25137;
                            }
                        }
                        '}' => {
                            if vec.pop().unwrap() != '{' {
                                acc += 1197;
                            }
                        }
                        c => vec.push(c),
                    }

                    acc
                })
            })
            .sum::<i32>()
            .to_string()
    }

    fn part2(&self, _input: &mut dyn std::io::Read, _extra_args: &[String]) -> String {
        let data = load_file(_input);
        let lines: Vec<String> = parse_lines::<String>(&data).collect();
        let mut outs: Vec<i64> = lines
            .iter()
            .map(|str| {
                let mut vec = Vec::new();

                for character in str.chars() {
                    match character {
                        ']' => {
                            if vec.pop().unwrap() != '[' {
                                return 0;
                            }
                        }
                        ')' => {
                            if vec.pop().unwrap() != '(' {
                                return 0;
                            }
                        }
                        '>' => {
                            if vec.pop().unwrap() != '<' {
                                return 0;
                            }
                        }
                        '}' => {
                            if vec.pop().unwrap() != '{' {
                                return 0;
                            }
                        }
                        c => vec.push(c),
                    }
                }
                vec.iter().rev().fold(0, |acc: i64, c| match c {
                    '[' => acc * 5 + 2,
                    '(' => acc * 5 + 1,
                    '<' => acc * 5 + 4,
                    '{' => acc * 5 + 3,
                    _ => acc,
                })
            })
            .filter(|x| *x > 0)
            .collect();

        outs.sort_unstable();

        outs[outs.len() / 2].to_string()
    }
}
