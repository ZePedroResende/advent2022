use crate::AoCDay;
pub struct Code;

fn check_velocity(mut dx: i64, mut dy: i64) -> Option<i64> {
    let (mut x, mut y, mut maxy) = (0, 0, 0);
    loop {
        x += dx;
        y += dy;
        dx -= dx.signum();
        dy -= 1;
        if y > maxy {
            maxy = y;
        }
        match (X_MIN <= x && x <= X_MAX, Y_MIN <= y && y <= Y_MAX) {
            (true, true) => return Some(maxy),
            (false, _) if dx == 0 => return None,
            (_, false) if dy < 0 && y < Y_MIN => return None,
            _ => {}
        }
    }
}

const X_MIN: i64 = 81;
const X_MAX: i64 = 129;
const Y_MIN: i64 = -150;
const Y_MAX: i64 = -108;

impl AoCDay for Code {
    fn part1(&self, _input: &mut dyn std::io::Read, _extra_argss: &[String]) -> String {
        let coords: Vec<(i64, i64)> = (0..=X_MAX).fold(Vec::new(), |acc, x| {
            (Y_MIN..1000).fold(acc, |mut acc_, y| {
                acc_.push((x, y));
                acc_
            })
        });

        let max_y = coords
            .into_iter()
            .filter_map(|(x, y)| check_velocity(x, y))
            .collect::<Vec<_>>();

        let out = *max_y.iter().max().unwrap();

        out.to_string()
    }
    fn part2(&self, _input: &mut dyn std::io::Read, _extra_args: &[String]) -> String {
        let coords: Vec<(i64, i64)> = (0..=X_MAX).fold(Vec::new(), |acc, x| {
            (Y_MIN..1000).fold(acc, |mut acc_, y| {
                acc_.push((x, y));
                acc_
            })
        });

        

        coords
            .into_iter()
            .filter_map(|(x, y)| check_velocity(x, y)).count().to_string()
    }
}
