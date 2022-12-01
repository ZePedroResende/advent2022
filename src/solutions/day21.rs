use crate::AoCDay;
use std::collections::HashMap;
use std::mem::swap;
pub struct Code;

type Cache = HashMap<(usize, usize, usize, usize), (usize, usize)>;

fn regular(mut pos1: usize, mut pos2: usize) -> usize {
    let (mut s1, mut s2, mut die, mut nrolls) = (0, 0, 1, 0);
    while s2 < 1000 {
        for _ in 0..3 {
            pos1 += die;
            die = (die + 1) % 100;
        }
        pos1 = 1 + (pos1 - 1) % 10;
        s1 += pos1;
        nrolls += 3;
        swap(&mut pos1, &mut pos2);
        swap(&mut s1, &mut s2);
    }
    nrolls * s1
}

fn quantum(cache: &mut Cache, s1: usize, s2: usize, pos1: usize, pos2: usize) -> (usize, usize) {
    if s2 >= 21 {
        return (0, 1);
    }
    if let Some(&score) = cache.get(&(s1, s2, pos1, pos2)) {
        return score;
    }

    let mut score = (0, 0);
    for (die, times) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
        let pos1 = 1 + (pos1 + die - 1) % 10;
        let (s1, s2) = quantum(cache, s2, s1 + pos1, pos2, pos1);
        score = (score.0 + s2 * times, score.1 + s1 * times);
    }

    cache.insert((s1, s2, pos1, pos2), score);
    score
}

impl AoCDay for Code {
    fn part1(&self, _input: &mut dyn std::io::Read, _extra_argss: &[String]) -> String {
        quantum(&mut HashMap::new(), 0, 0, 1, 5);
        regular(1, 5).to_string()
    }
    fn part2(&self, _input: &mut dyn std::io::Read, _extra_args: &[String]) -> String {
        let (a, b) = quantum(&mut HashMap::new(), 0, 0, 1, 5);
        std::cmp::max(a, b).to_string()
    }
}
