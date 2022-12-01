use crate::{load_file, AoCDay};
pub struct Code;

fn median(numbers: &mut Vec<i32>) -> i32 {
    numbers.sort_unstable();
    let mid = numbers.len() / 2;
    numbers[mid]
}

impl AoCDay for Code {
    fn part1(&self, _input: &mut dyn std::io::Read, _extra_argss: &[String]) -> String {
        let data = load_file(_input);
        let mut numbers: Vec<i32> = data
            .lines().next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<i32>().expect("Not a number"))
            .collect();

        let median = median(&mut numbers);
        numbers
            .iter()
            .map(|n| (n - median).abs())
            .sum::<i32>()
            .to_string()
    }
    fn part2(&self, _input: &mut dyn std::io::Read, _extra_args: &[String]) -> String {
        let data = load_file(_input);
        let numbers: Vec<i32> = data
            .lines().next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<i32>().expect("Not a number"))
            .collect();

        let mean = numbers.iter().sum::<i32>() / numbers.len() as i32;

        numbers
            .iter()
            .map(|n| ((n - mean).abs()) * ((n - mean).abs() + 1) / 2)
            .sum::<i32>()
            .to_string()
    }
}
