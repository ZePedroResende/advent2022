use crate::{load_file, parse_lines, AoCDay};
pub struct Code;

fn calculate(matrix: &mut Vec<Vec<usize>>, c: usize, r: usize) -> usize {
    if matrix[r][c] < 10 || matrix[r][c] == 0 {
        return 0;
    }
    let mut amount = 1;
    matrix[r][c] = 0;

    let mut neighbours = vec![(r, c + 1), (r + 1, c), (r + 1, c + 1)];

    if r > 0 {
        neighbours.push((r - 1, c));
        neighbours.push((r - 1, c + 1));
    }

    if c > 0 {
        neighbours.push((r, c - 1));
        neighbours.push((r + 1, c - 1));
    }

    if r > 0 && c > 0 {
        neighbours.push((r - 1, c - 1));
    }

    for &(r, c) in &neighbours {
        //if r >= matrix.len() && c >= matrix[0].len()
        if matrix.get(r).and_then(|row| row.get(c)).is_none() {
            continue;
        }
        if matrix[r][c] != 0 {
            matrix[r][c] += 1;
            amount += calculate(matrix, c, r);
        }
    }

    amount
}

fn tick(matrix: &mut Vec<Vec<usize>>) -> usize {
    let mut amount = 0;

    for line in matrix.iter_mut() {
        for item in line.iter_mut() {
            *item += 1
        }
    }

    for i in 0..10 {
        for j in 0..10 {
            amount += calculate(matrix, i, j);
        }
    }

    amount
}

impl AoCDay for Code {
    fn part1(&self, _input: &mut dyn std::io::Read, _extra_argss: &[String]) -> String {
        let data = load_file(_input);
        let mut lines: Vec<Vec<usize>> = parse_lines::<String>(&data)
            .map(|str| {
                str.chars()
                    .map(|char| char.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();

        let sum: usize = (0..100).map(|_| tick(&mut lines)).sum::<usize>();

        sum.to_string()
    }
    fn part2(&self, _input: &mut dyn std::io::Read, _extra_args: &[String]) -> String {
        let data = load_file(_input);
        let mut lines: Vec<Vec<usize>> = parse_lines::<String>(&data)
            .map(|str| {
                str.chars()
                    .map(|char| char.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();

        let sum: usize = (0..).find(|_| tick(&mut lines) == 100).unwrap() + 1;

        sum.to_string()
    }
}
