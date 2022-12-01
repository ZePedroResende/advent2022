use crate::{load_file, parse_lines, AoCDay};
use std::collections::HashSet;
pub struct Code;
fn check_proyimitx(y: usize, x: usize, vec: &[Vec<u32>]) -> bool {
    let mut vector = Vec::new();
    if y != 0 {
        vector.push(vec[y - 1][x]);
    }
    if y != (vec.len() - 1) {
        vector.push(vec[y + 1][x]);
    }

    if x != 0 {
        vector.push(vec[y][x - 1]);
    }
    if x != (vec[0].len() - 1) {
        vector.push(vec[y][x + 1]);
    }

    match vector.iter().min() {
        Some(a) => a > &vec[y][x],
        None => false,
    }
}

fn neighbors(
    y: usize,
    x: usize,
    width: usize,
    height: usize,
) -> impl Iterator<Item = (usize, usize)> {
    println!("{} {} {} {}", y, x, width, height);
    let mut vector = vec![(y < height - 1, (y + 1, x)), (x < width - 1, (y, x + 1))];
    if y != 0 {
        vector.push((y > 0, (y - 1, x)));
    }

    if x != 0 {
        vector.push((x > 0, (y, x - 1)));
    }

    vector
        .into_iter()
        .filter_map(|(cond, value)| if cond { Some(value) } else { None })
}

impl AoCDay for Code {
    fn part1(&self, _input: &mut dyn std::io::Read, _eytra_argss: &[String]) -> String {
        let data = load_file(_input);
        let lines: Vec<Vec<u32>> = parse_lines::<String>(&data)
            .map(|str| str.chars().map(|char| char.to_digit(10).unwrap()).collect())
            .collect();

        let mut out = Vec::new();
        lines.iter().enumerate().for_each(|(i, y)| {
            y.iter().enumerate().for_each(|(j, _x)| {
                if check_proyimitx(i, j, &lines) {
                    out.push(lines[i][j] + 1);
                }
            });
        });

        out.iter().sum::<u32>().to_string()
    }

    fn part2(&self, _input: &mut dyn std::io::Read, _extra_args: &[String]) -> String {
        let data = load_file(_input);
        let lines: Vec<Vec<u32>> = parse_lines::<String>(&data)
            .map(|str| str.chars().map(|char| char.to_digit(10).unwrap()).collect())
            .collect();

        let mut out = Vec::new();
        lines.iter().enumerate().for_each(|(i, y)| {
            y.iter().enumerate().for_each(|(j, _x)| {
                if check_proyimitx(i, j, &lines) {
                    out.push((i, j));
                }
            });
        });

        let product: Vec<HashSet<(usize, usize)>> = out
            .into_iter()
            .map(|(y, x)| {
                let mut stack = vec![(y, x)];
                let mut visited = HashSet::new();
                while let Some((y, x)) = stack.pop() {
                    if !visited.insert((y, x)) {
                        continue;
                    }
                    neighbors(y, x, lines[0].len(), lines.len())
                        .filter(|&(ny, nx)| lines[ny][nx] != 9 && lines[ny][nx] > lines[y][x])
                        .for_each(|node| {
                            stack.push(node);
                        });
                }
                visited
            })
            .collect();

        let mut lengths: Vec<usize> = product.iter().map(|basin| basin.len()).collect();
        lengths.sort_unstable();
        let o = lengths.iter().rev().take(3).product::<usize>();

        o.to_string()
    }
}
