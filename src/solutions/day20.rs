use crate::{load_file, AoCDay};
pub struct Code;
fn updated_tile(grid: &[Vec<u8>], r: usize, c: usize, val: u8, map: &[u8]) -> u8 {
    let ns = [
        (r - 1, c - 1),
        (r - 1, c),
        (r - 1, c + 1),
        (r, c - 1),
        (r, c),
        (r, c + 1),
        (r + 1, c - 1),
        (r + 1, c),
        (r + 1, c + 1),
    ];
    let i = ns.iter().fold(0, |n, &(r, c)| {
        let x = *grid.get(r).and_then(|row| row.get(c)).unwrap_or(&val) as usize;
        n << 1 | x
    });
    (map[i] == b'#') as u8
}

fn enhance(grid: &[Vec<u8>], val: u8, map: &[u8]) -> Vec<Vec<u8>> {
    let mut ans = vec![vec![0; grid[0].len() + 2]; grid.len() + 2];

    let coords = (0..ans.len()).fold(Vec::new(), |acc, x| {
        (0..ans[0].len()).fold(acc, |mut acc_, y| {
            acc_.push((x, y));
            acc_
        })
    });

    for (r, c) in coords {
        ans[r][c] = updated_tile(grid, r - 1, c - 1, val, map);
    }

    ans
}

impl AoCDay for Code {
    fn part1(&self, input: &mut dyn std::io::Read, _extra_argss: &[String]) -> String {
        let string = load_file(input);
        let (map_string, grid_string) = string.split_once("\n\n").unwrap();
        let map = map_string.as_bytes();
        let mut grid = grid_string
            .lines()
            .map(|l| l.chars().map(|c| (c == '#') as u8).collect())
            .collect::<Vec<_>>();

        for i in 0..2 {
            grid = enhance(&grid, i & 1, map)
        }

        grid.iter()
            .flatten()
            .filter(|&&b| b == 1)
            .count()
            .to_string()
    }

    fn part2(&self, input: &mut dyn std::io::Read, _extra_args: &[String]) -> String {
        let string = load_file(input);
        let (map_string, grid_string) = string.split_once("\n\n").unwrap();
        let map = map_string.as_bytes();
        let mut grid = grid_string
            .lines()
            .map(|l| l.chars().map(|c| (c == '#') as u8).collect())
            .collect::<Vec<_>>();

        for i in 0..2 {
            grid = enhance(&grid, i & 1, map)
        }

        for i in 2..50 {
            grid = enhance(&grid, i & 1, map)
        }

        grid.iter()
            .flatten()
            .filter(|&&b| b == 1)
            .count()
            .to_string()
    }
}
