use crate::{load_file, AoCDay};
use std::cmp::{max, min};
use std::collections::HashMap;
pub struct Code;

type Coords = ((i64, i64), (i64, i64), (i64, i64));
type Cube = (bool, (i64, i64), (i64, i64), (i64, i64));

fn reboot(cubes: &mut HashMap<Coords, i64>, coords: Cube) -> HashMap<Coords, i64> {
    let (status, (x0, x1), (y0, y1), (z0, z1)) = coords;

    let status_i = if status { 1 } else { -1 };

    for (((kx0, kx1), (ky0, ky1), (kz0, kz1)), value) in cubes.clone().into_iter() {
        let ix0 = max(x0, kx0);
        let ix1 = min(x1, kx1);

        let iy0 = max(y0, ky0);
        let iy1 = min(y1, ky1);

        let iz0 = max(z0, kz0);
        let iz1 = min(z1, kz1);

        if ix0 <= ix1 && iy0 <= iy1 && iz0 <= iz1 {
            *cubes
                .entry(((ix0, ix1), (iy0, iy1), (iz0, iz1)))
                .or_insert(0) -= value;
        }
    }

    if status_i > 0 {
        *cubes.entry(((x0, x1), (y0, y1), (z0, z1))).or_insert(0) += status_i;
    }

    cubes.clone()
}

impl AoCDay for Code {
    fn part1(&self, _input: &mut dyn std::io::Read, _extra_argss: &[String]) -> String {
        let lines = load_file(_input);

        

        let filtered_cubes: Vec<Cube> = lines
            .lines()
            .map(|l| {
                let (status, coords) = l.split_once(' ').unwrap();
                let coords: Vec<(i64, i64)> = coords
                    .split(',')
                    .map(|s| {
                        let values: Vec<i64> =
                            s[2..].split("..").map(|x| x.parse().unwrap()).collect();
                        (values[0], values[1])
                    })
                    .collect();

                (status == "on", coords[0], coords[1], coords[2])
            })
            .filter(|cube| {
                cube.1 .0 <= 50
                    && cube.1 .1 >= -50
                    && cube.2 .0 <= 50
                    && cube.2 .1 >= -50
                    && cube.3 .0 <= 50
                    && cube.3 .1 >= -50
            })
            .collect::<Vec<Cube>>();

        let out = filtered_cubes
            .iter()
            .fold(HashMap::new(), |mut acc, cube| reboot(&mut acc, *cube));

        out.into_iter()
            .fold(0, |acc, (((x0, x1), (y0, y1), (z0, z1)), value)| {
                acc + (x1 - x0 + 1) * (y1 - y0 + 1) * (z1 - z0 + 1) * value
            })
            .to_string()
    }

    fn part2(&self, _input: &mut dyn std::io::Read, _extra_args: &[String]) -> String {
        let lines = load_file(_input);

        let cubes: Vec<Cube> = lines
            .lines()
            .map(|l| {
                let (status, coords) = l.split_once(' ').unwrap();
                let coords: Vec<(i64, i64)> = coords
                    .split(',')
                    .map(|s| {
                        let values: Vec<i64> =
                            s[2..].split("..").map(|x| x.parse().unwrap()).collect();
                        (values[0], values[1])
                    })
                    .collect();

                (status == "on", coords[0], coords[1], coords[2])
            })
            .collect::<Vec<Cube>>();

        let out = cubes
            .iter()
            .fold(HashMap::new(), |mut acc, cube| reboot(&mut acc, *cube));

        out.into_iter()
            .fold(0, |acc, (((x0, x1), (y0, y1), (z0, z1)), value)| {
                acc + (x1 - x0 + 1) * (y1 - y0 + 1) * (z1 - z0 + 1) * value
            })
            .to_string()
    }
}
