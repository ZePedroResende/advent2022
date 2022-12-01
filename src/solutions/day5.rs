use crate::{load_file, parse_lines, AoCDay};
use std::collections::HashMap;
use std::str::FromStr;
pub struct Code;

#[derive(Debug, Clone, Copy)]
struct Line {
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl FromStr for Line {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line: Vec<Vec<i64>> = s
            .split(" -> ")
            .map(|v| {
                v.split(',')
                    .map(|e| e.parse::<i64>().expect("Not a number"))
                    .collect()
            })
            .collect();

        Ok(Self {
            x1: line[0][0],
            x2: line[1][0],
            y1: line[0][1],
            y2: line[1][1],
        })
    }
}

impl Line {
    fn calculate_points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        let mut add_x: i64 = 0;
        let mut add_y: i64 = 0;

        if self.x1 != self.x2 {
            add_x = if self.x1 > self.x2 { -1 } else { 1 };
        }
        if self.y1 != self.y2 {
            add_y = if self.y1 > self.y2 { -1 } else { 1 };
        }

        let mut start_x = self.x1;
        let end_x = self.x2;
        let mut start_y = self.y1;
        let end_y = self.y2;

        if add_x != 0 && add_y != 0 {
            return points;
        }

        while (start_x != end_x) || (start_y != end_y) {
            points.push(Point {
                x: start_x,
                y: start_y,
            });

            start_x += add_x;
            start_y += add_y;
        }
        points.push(Point {
            x: start_x,
            y: start_y,
        });

        points
    }
    fn calculate_points_part2(&self) -> Vec<Point> {
        let mut points = Vec::new();
        let mut add_x: i64 = 0;
        let mut add_y: i64 = 0;

        if self.x1 != self.x2 {
            add_x = if self.x1 > self.x2 { -1 } else { 1 };
        }
        if self.y1 != self.y2 {
            add_y = if self.y1 > self.y2 { -1 } else { 1 };
        }

        let mut start_x = self.x1;
        let end_x = self.x2;
        let mut start_y = self.y1;
        let end_y = self.y2;

        while (start_x != end_x) || (start_y != end_y) {
            points.push(Point {
                x: start_x,
                y: start_y,
            });

            start_x += add_x;
            start_y += add_y;
        }
        points.push(Point {
            x: start_x,
            y: start_y,
        });

        points
    }
}

impl AoCDay for Code {
    fn part1(&self, _input: &mut dyn std::io::Read, _extra_argss: &[String]) -> String {
        let data = load_file(_input);
        let lines: Vec<Line> = parse_lines::<Line>(&data).collect();
        let mut map: HashMap<Point, i64> = std::collections::HashMap::new();

        lines.iter().for_each(|line| {
            let points = line.calculate_points();

            points.iter().for_each(|point| {
                let count = map.entry(*point).or_insert(0);
                *count += 1;
            })
        });

        let value: i64 = map
            .values()
            .fold(0, |acc, value| acc + ((*value > 1) as i64));

        value.to_string()
    }
    fn part2(&self, _input: &mut dyn std::io::Read, _extra_args: &[String]) -> String {
        let data = load_file(_input);
        let lines: Vec<Line> = parse_lines::<Line>(&data).collect();
        let mut map: HashMap<Point, i64> = std::collections::HashMap::new();

        lines.iter().for_each(|line| {
            let points = line.calculate_points_part2();

            points.iter().for_each(|point| {
                let count = map.entry(*point).or_insert(0);
                *count += 1;
            })
        });

        let value: i64 = map
            .values()
            .fold(0, |acc, value| acc + ((*value > 1) as i64));

        value.to_string()
    }
}
