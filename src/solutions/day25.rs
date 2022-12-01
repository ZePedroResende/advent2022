use crate::{load_file, AoCDay};
pub struct Code;

fn change(grid: &mut Vec<Vec<u8>>) -> usize {
    let mut count = 0;

    let column_length = grid.len();
    let row_length = grid[0].len();

    for (character, (x, y)) in [(b'>', (0, 1)), (b'v', (1, 0))] {
        let mut next_grid = grid.clone();
        for (i, row) in grid.iter().enumerate() {
            for (j, item) in row.iter().enumerate() {
                if *item == character && grid[(i + x) % column_length][(j + y) % row_length] == b'.'
                {
                    next_grid[(i + x) % column_length][(j + y) % row_length] = character;
                    next_grid[i][j] = b'.';
                    count += 1;
                }
            }
        }
        *grid = next_grid;
    }

    count
}

impl AoCDay for Code {
    fn part1(&self, _input: &mut dyn std::io::Read, _extra_argss: &[String]) -> String {
        let lines = load_file(_input);
        let mut grid: Vec<Vec<u8>> = lines.lines().map(|l| l.bytes().collect()).collect();

        for tick in 1.. {
            let changes = change(&mut grid);

            if changes == 0 {
                return tick.to_string();
            }
        }

        String::from("0")
    }
    fn part2(&self, _input: &mut dyn std::io::Read, _extra_args: &[String]) -> String {
        todo!()
    }
}
