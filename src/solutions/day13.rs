use crate::{load_file, parse_lines, AoCDay};
pub struct Code;
use std::collections::HashSet;

fn calculate(
    grid: &HashSet<(usize, usize)>,
    (axis, num): &(&str, usize),
) -> HashSet<(usize, usize)> {
    let mut hash = HashSet::new();
    for (x, y) in grid.iter() {
        hash.insert((
            if *axis == "x" {
                if *x > *num {
                    2 * num - x
                } else {
                    *x
                }
            } else {
                *x
            },
            if *axis == "y" {
                if *y > *num {
                    2 * num - y
                } else {
                    *y
                }
            } else {
                *y
            },
        ));
    }

    hash
}

impl AoCDay for Code {
    fn part1(&self, _input: &mut dyn std::io::Read, _extra_argss: &[String]) -> String {
        let data = load_file(_input);

        let (cords_str, fold_str) = data.split_once("\n\n").unwrap();
        let cords_data: Vec<String> = parse_lines::<String>(cords_str).collect();
        let fold: Vec<(&str, usize)> = fold_str
            .lines()
            .map(|l| {
                let (_, instruction) = l.rsplit_once(' ').unwrap();
                let (axis, num) = instruction.rsplit_once('=').unwrap();
                (axis, num.parse().unwrap())
            })
            .collect();

        let cords: HashSet<(usize, usize)> = cords_data
            .iter()
            .map(|l| {
                let (x, y) = l.split_once(",").unwrap();
                (x.parse().unwrap(), y.parse().unwrap())
            })
            .collect();

        println!("{:?}", fold);

        let number = calculate(&cords, &fold[0]);
        println!("{:?}", cords.len());
        println!("{:?}", number);
        number.len().to_string()
    }
    fn part2(&self, _input: &mut dyn std::io::Read, _extra_args: &[String]) -> String {
        let data = load_file(_input);

        let (cords_str, fold_str) = data.split_once("\n\n").unwrap();
        let cords_data: Vec<String> = parse_lines::<String>(cords_str).collect();
        let fold: Vec<(&str, usize)> = fold_str
            .lines()
            .map(|l| {
                let (_, instruction) = l.rsplit_once(' ').unwrap();
                let (axis, num) = instruction.rsplit_once('=').unwrap();
                (axis, num.parse().unwrap())
            })
            .collect();

        let cords: HashSet<(usize, usize)> = cords_data
            .iter()
            .map(|l| {
                let (x, y) = l.split_once(",").unwrap();
                (x.parse().unwrap(), y.parse().unwrap())
            })
            .collect();

        println!("{:?}", fold);

        let out = fold.iter().fold(cords, |acc, f| calculate(&acc, f));

        (0..=6).for_each(|y: usize| {
            (0..=40).for_each(|x: usize| {
                if out.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(" ");
                }
            });
            println!();
        });

        let number = 1;
        number.to_string()
    }
}
