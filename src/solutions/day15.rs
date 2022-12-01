use crate::{load_file, parse_lines, AoCDay};
use std::collections::{BinaryHeap, HashMap};
pub struct Code;

fn neighbours((x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let mut vector = vec![(x, y + 1), (x + 1, y)];
    if y != 0 {
        vector.push((x, y - 1));
    }

    if x != 0 {
        vector.push((x - 1, y));
    }

    vector
}

fn djks(matrix: &[Vec<usize>]) -> i64 {
    let end = (matrix.len() - 1, matrix[0].len() - 1);

    let mut distances = HashMap::new();
    let mut to_visit = BinaryHeap::new();

    distances.insert((0, 0), 0);
    to_visit.push((0, 0, 0));

    while let Some((distance, x, y)) = to_visit.pop() {
        if (x, y) == end {
            return -distance;
        }

        if let Some(current_distance) = distances.get(&(x, y)) {
            if -distance > *current_distance {
                continue;
            }
        }

        let neighbours = neighbours((x, y));

        for coord in neighbours.iter() {
            let (n_x, n_y) = coord;
            if matrix.get(*n_x).and_then(|row| row.get(*n_y)).is_none() {
                continue;
            }

            let new_distance = distance.abs() + matrix[*n_x][*n_y] as i64;

            let is_shorter = distances
                .get(coord)
                .map_or(true, |&current| new_distance < current);

            if is_shorter {
                distances.insert(*coord, new_distance);
                to_visit.push((-new_distance, *n_x, *n_y))
            }
        }
    }

    *distances.get(&end).unwrap()
}

impl AoCDay for Code {
    fn part1(&self, _input: &mut dyn std::io::Read, _extra_argss: &[String]) -> String {
        let data = load_file(_input);
        let matrix: Vec<Vec<usize>> = parse_lines::<String>(&data)
            .map(|str| {
                str.chars()
                    .map(|char| char.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();

        djks(&matrix).to_string()
    }
    fn part2(&self, _input: &mut dyn std::io::Read, _extra_args: &[String]) -> String {
        let data = load_file(_input);
        let matrix: Vec<Vec<usize>> = parse_lines::<String>(&data)
            .map(|str| {
                str.chars()
                    .map(|char| char.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();
        let matrix5 = (0..(5 * matrix.len()))
            .map(|x| {
                (0..(5 * matrix[0].len()))
                    .map(|y| {
                        let x_lvl = x / matrix.len();
                        let y_lvl = y / matrix[0].len();
                        let cost = matrix[x % matrix.len()][y % matrix[0].len()] + x_lvl + y_lvl;
                        if cost < 10 {
                            cost
                        } else {
                            cost - 9
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        djks(&matrix5).to_string()
    }
}
