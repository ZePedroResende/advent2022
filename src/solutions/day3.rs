use crate::{load_file, parse_lines, AoCDay};
pub struct Code;
//impl FromStr for Vec<u8>{
//    type Err = ();
//    fn from_str(string: &str) -> Result<Self,()> {
//        for char in string.iter()
//        }
//    }
//}
//
//

//fn part1(input: Vec<String>) -> u64 {
//    let cols = input.lines().next().ok_or("empty input")?.len();
//    let rows = input.lines().count();
//    let (gamma, epsilon) = (0..cols)
//        .map(|i| {
//            input
//                .lines()
//                .filter(|line| line.as_bytes()[i] == b'1')
//                .count()
//        })
//        .fold((0, 0), |(gamma, epsilon), ones| {
//            if ones * 2 > rows {
//                ((gamma << 1) | 1, epsilon << 1)
//            } else {
//                (gamma << 1, (epsilon << 1) | 1)
//            }
//        });
//    gamma * epsilon
//}
//
//fn oxygen_co2(input: Vec<String>) -> u64 {
//    let rating = |most_common: bool| -> Result<u32> {
//        let mut seq: Vec<_> = input.lines().collect();
//        let mut col = 0;
//        while seq.len() > 1 {
//            let ones = seq.iter().filter(|l| l.as_bytes()[col] == b'1').count();
//            let bit = match (most_common, ones * 2 >= seq.len()) {
//                (true, true) | (false, false) => b'1',
//                _ => b'0',
//            };
//            seq = seq
//                .into_iter()
//                .filter(|l| l.as_bytes()[col] == bit)
//                .collect();
//            col += 1;
//        }
//        Ok(u32::from_str_radix(seq.first().ok_or("empty input")?, 2)?)
//    };
//    let (oxygen, co2) = (rating(true)?, rating(false)?);
//    Ok(oxygen * co2)
//}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}
impl AoCDay for Code {
    fn part1(&self, _input: &mut dyn std::io::Read, _extra_argss: &[String]) -> String {
        let data = load_file(_input);
        let lines: Vec<Vec<i64>> = parse_lines::<String>(&data)
            .map(|str| {
                str.chars()
                    .map(|char| {
                        if char.to_digit(10).unwrap() == 0 {
                            -1
                        } else {
                            1
                        }
                    })
                    .collect()
            })
            .collect();

        let vec: Vec<u8> = transpose(lines)
            .iter()
            .map(|line| (line.iter().sum::<i64>() > 0) as u8)
            .collect();

        let gamma = vec.iter().fold(0, |acc, &b| acc * 2 + b as u64);
        let epsilon = !gamma & 0xfff;

        (gamma * epsilon).to_string()
    }
    fn part2(&self, _input: &mut dyn std::io::Read, _extra_args: &[String]) -> String {
        let mut result = Vec::new();
        let mut result2 = Vec::new();
        let data = load_file(_input);
        let lines: Vec<Vec<i64>> = parse_lines::<String>(&data)
            .map(|str| {
                str.chars()
                    .map(|char| {
                        if char.to_digit(10).unwrap() == 0 {
                            -1
                        } else {
                            1
                        }
                    })
                    .collect()
            })
            .collect();

        let vec: Vec<u8> = transpose(lines)
            .iter()
            .map(|line| (line.iter().sum::<i64>() > 0) as u8)
            .collect();

        for (i, num) in vec.iter().rev().enumerate() {
            //            let tmp = num ^ 0;
            result.retain(|x: &u8| (x >> i) & 1 == *num);
            if result.len() == 1 {
                break;
            }
        }

        for (i, num) in vec.iter().rev().enumerate() {
            //           let tmp = num ^ 1;
            result2.retain(|x: &u8| (x >> i) & 1 == *num);
            if result2.len() == 1 {
                break;
            }
        }
        (result[0] * result2[0]).to_string()
    }
}
