use crate::{load_file, AoCDay};
use std::collections::HashSet;
pub struct Code;

fn rotate([x, y, z]: [i32; 3], rot: u8) -> [i32; 3] {
    match rot {
        0 => [x, y, z],
        1 => [x, z, -y],
        2 => [x, -y, -z],
        3 => [x, -z, y],
        4 => [y, x, -z],
        5 => [y, z, x],
        6 => [y, -x, z],
        7 => [y, -z, -x],
        8 => [z, x, y],
        9 => [z, y, -x],
        10 => [z, -x, -y],
        11 => [z, -y, x],
        12 => [-x, y, -z],
        13 => [-x, z, y],
        14 => [-x, -y, z],
        15 => [-x, -z, -y],
        16 => [-y, x, z],
        17 => [-y, z, -x],
        18 => [-y, -x, -z],
        19 => [-y, -z, x],
        20 => [-z, x, -y],
        21 => [-z, y, x],
        22 => [-z, -x, y],
        23 => [-z, -y, -x],
        _ => unreachable!(),
    }
}

fn merge_scan(total_scan: &mut HashSet<[i32; 3]>, scan: &[[i32; 3]]) -> Option<[i32; 3]> {
    for rot in 0..24 {
        let rotated_scan = scan.iter().map(|&v| rotate(v, rot)).collect::<Vec<_>>();

        let coords = total_scan.iter().fold(Vec::new(), |acc, x| {
            rotated_scan.iter().fold(acc, |mut acc_, y| {
                acc_.push((x, y));
                acc_
            })
        });

        let distances = coords
            .iter()
            .map(|([x1, y1, z1], [x2, y2, z2])| [x1 - x2, y1 - y2, z1 - z2]);

        for [dx, dy, dz] in distances {
            let translated = rotated_scan
                .iter()
                .map(|[x, y, z]| [x + dx, y + dy, z + dz]);

            if translated
                .clone()
                .filter(|v| total_scan.contains(v))
                .count()
                >= 12
            {
                total_scan.extend(translated);
                return Some([dx, dy, dz]);
            }
        }
    }
    None
}

impl AoCDay for Code {
    fn part1(&self, input: &mut dyn std::io::Read, _extra_argss: &[String]) -> String {
        let mut scans = load_file(input)
            .split("\n\n")
            .map(|s| {
                s.lines()
                    .skip(1)
                    .map(|l| {
                        let (a, tmp) = l.split_once(',').unwrap();
                        let (b, c) = tmp.split_once(',').unwrap();
                        [a.parse().unwrap(), b.parse().unwrap(), c.parse().unwrap()]
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut total_scan = scans.remove(0).into_iter().collect::<HashSet<_>>();

        let mut dists = Vec::new();

        while !scans.is_empty() {
            for i in (0..scans.len()).rev() {
                if let Some(d) = merge_scan(&mut total_scan, &scans[i]) {
                    dists.push(d);
                    scans.swap_remove(i);
                }
            }
        }

        total_scan.len().to_string()
    }

    fn part2(&self, input: &mut dyn std::io::Read, _extra_args: &[String]) -> String {
        let mut scans = load_file(input)
            .split("\n\n")
            .map(|s| {
                s.lines()
                    .skip(1)
                    .map(|l| {
                        let (a, tmp) = l.split_once(',').unwrap();
                        let (b, c) = tmp.split_once(',').unwrap();
                        [a.parse().unwrap(), b.parse().unwrap(), c.parse().unwrap()]
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut total_scan = scans.remove(0).into_iter().collect::<HashSet<_>>();

        let mut dists = Vec::new();

        while !scans.is_empty() {
            for i in (0..scans.len()).rev() {
                if let Some(d) = merge_scan(&mut total_scan, &scans[i]) {
                    dists.push(d);
                    scans.swap_remove(i);
                }
            }
        }

        let tuples = dists
            .iter()
            .enumerate()
            .flat_map(|(i, a)| dists[i + 1..].iter().map(move |b| (a, b)));

        tuples
            .map(|([x1, y1, z1], [x2, y2, z2])| (x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs())
            .max()
            .unwrap()
            .to_string()
    }
}
