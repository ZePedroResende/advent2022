use crate::{load_file, AoCDay};
use std::collections::HashMap;
pub struct Code;

#[derive(Clone, Copy, Debug)]
enum Reg {
    Reg(usize),
    Val(i64),
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Inp(usize),
    Add(usize, Reg),
    Mul(usize, Reg),
    Div(usize, Reg),
    Mod(usize, Reg),
    Eql(usize, Reg),
}

impl Reg {
    fn from_str(str: &str) -> Reg {
        match str {
            "w" => Self::Reg(0),
            "x" => Self::Reg(1),
            "y" => Self::Reg(2),
            "z" => Self::Reg(3),
            _ => Self::Val(str.parse().unwrap()),
        }
    }

    fn get_value(&self, register: &[i64; 4]) -> i64 {
        match self {
            Reg::Reg(val) => register[*val],
            Reg::Val(val) => *val,
        }
    }

    fn get(&self) -> usize {
        match self {
            Reg::Reg(value) => *value,
            _ => panic!(),
        }
    }
}

impl Instruction {
    fn run(&self, regs: &mut [i64; 4]) {
        match self {
            Instruction::Add(a, b) => regs[*a] += b.get_value(regs),
            Instruction::Mul(a, b) => regs[*a] *= b.get_value(regs),
            Instruction::Div(a, b) => regs[*a] /= b.get_value(regs),
            Instruction::Mod(a, b) => regs[*a] %= b.get_value(regs),
            Instruction::Eql(a, b) => regs[*a] = (regs[*a] == b.get_value(regs)) as i64,
            Instruction::Inp(_) => unreachable!(),
        }
    }
}

fn run(
    instructions: &[Vec<Instruction>],
    visited: &mut HashMap<(i64, usize), Option<i64>>,
    block: usize,
    z: i64,
    range: &[i64],
) -> Option<i64> {
    if let Some(&answer) = visited.get(&(z, block)) {
        return answer;
    }

    for &digit in range {
        let mut regs = [digit, 0, 0, z];

        for &inst in &instructions[block] {
            inst.run(&mut regs);
        }

        let z = regs[3];

        if block + 1 == instructions.len() {
            if z == 0 {
                visited.insert((z, block), Some(digit));
                return Some(digit);
            }

            continue;
        }

        if let Some(best) = run(instructions, visited, block + 1, z, range) {
            visited.insert((z, block), Some(best * 10 + digit));
            return Some(best * 10 + digit);
        }
    }

    visited.insert((z, block), None);
    None
}

impl AoCDay for Code {
    fn part1(&self, _input: &mut dyn std::io::Read, _extra_argss: &[String]) -> String {
        let lines = load_file(_input);

        let instructions: Vec<Instruction> = lines
            .lines()
            .map(|l| {
                let mut splits = l.split(' ');
                let instruction = splits.next().unwrap();

                match instruction {
                    "inp" => Instruction::Inp(Reg::from_str(splits.next().unwrap()).get()),
                    "add" => Instruction::Add(
                        Reg::from_str(splits.next().unwrap()).get(),
                        Reg::from_str(splits.next().unwrap()),
                    ),
                    "mul" => Instruction::Mul(
                        Reg::from_str(splits.next().unwrap()).get(),
                        Reg::from_str(splits.next().unwrap()),
                    ),
                    "div" => Instruction::Div(
                        Reg::from_str(splits.next().unwrap()).get(),
                        Reg::from_str(splits.next().unwrap()),
                    ),
                    "mod" => Instruction::Mod(
                        Reg::from_str(splits.next().unwrap()).get(),
                        Reg::from_str(splits.next().unwrap()),
                    ),
                    "eql" => Instruction::Eql(
                        Reg::from_str(splits.next().unwrap()).get(),
                        Reg::from_str(splits.next().unwrap()),
                    ),
                    _ => unreachable!(),
                }
            })
            .collect();

        let out: Vec<Vec<Instruction>> = instructions
            .chunks(18)
            .into_iter()
            .map(|instructions_chunk| {
                let mut vector: Vec<Instruction> = instructions_chunk.into();

                vector.remove(0);
                vector
            })
            .collect();

        let range: Vec<i64> = (1..=9).rev().collect();
        let out = run(
            &out,
            &mut HashMap::<(i64, usize), Option<i64>>::new(),
            0,
            0,
            &range,
        );

        out.unwrap().to_string().chars().rev().collect()
    }
    fn part2(&self, _input: &mut dyn std::io::Read, _extra_args: &[String]) -> String {
        let lines = load_file(_input);

        let instructions: Vec<Instruction> = lines
            .lines()
            .map(|l| {
                let mut splits = l.split(' ');
                let instruction = splits.next().unwrap();

                match instruction {
                    "inp" => Instruction::Inp(Reg::from_str(splits.next().unwrap()).get()),
                    "add" => Instruction::Add(
                        Reg::from_str(splits.next().unwrap()).get(),
                        Reg::from_str(splits.next().unwrap()),
                    ),
                    "mul" => Instruction::Mul(
                        Reg::from_str(splits.next().unwrap()).get(),
                        Reg::from_str(splits.next().unwrap()),
                    ),
                    "div" => Instruction::Div(
                        Reg::from_str(splits.next().unwrap()).get(),
                        Reg::from_str(splits.next().unwrap()),
                    ),
                    "mod" => Instruction::Mod(
                        Reg::from_str(splits.next().unwrap()).get(),
                        Reg::from_str(splits.next().unwrap()),
                    ),
                    "eql" => Instruction::Eql(
                        Reg::from_str(splits.next().unwrap()).get(),
                        Reg::from_str(splits.next().unwrap()),
                    ),
                    _ => unreachable!(),
                }
            })
            .collect();

        let out: Vec<Vec<Instruction>> = instructions
            .chunks(18)
            .into_iter()
            .map(|instructions_chunk| {
                let mut vector: Vec<Instruction> = instructions_chunk.into();

                vector.remove(0);
                vector
            })
            .collect();

        let range: Vec<i64> = (1..=9).collect();
        let out = run(
            &out,
            &mut HashMap::<(i64, usize), Option<i64>>::new(),
            0,
            0,
            &range,
        );

        out.unwrap().to_string().chars().rev().collect()
    }
}
