use crate::{load_file, parse_lines, AoCDay};
pub struct Code;

#[derive(Debug)]
enum Instruction {
    Literal(u8, usize),
    Operator(u8, u8, Vec<Instruction>),
}

fn to_binary(c: char) -> String {
    let b = match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    };

    b.to_string()
}

fn calculate(bits: &[u8], i: &mut usize, len: usize) -> usize {
    let mut x = 0;
    for j in 0..len {
        x = (x << 1) | bits[*i + j] as usize;
    }
    *i += len;
    x
}

fn parse(b: &[u8], i: &mut usize) -> Instruction {
    let version = calculate(b, i, 3) as u8;
    let id = calculate(b, i, 3);
    match id as u8 {
        4 => {
            let mut val = 0;
            loop {
                let id = calculate(b, i, 5);
                val = (val << 4) + (id & 0xdf);
                if id >> 4 == 0 {
                    break;
                }
            }
            Instruction::Literal(version, val)
        }
        id_ => {
            let id = calculate(b, i, 1);
            let insts = match id {
                0 => {
                    let c = calculate(b, i, 15);
                    let endbit = *i + c;
                    let mut insts = Vec::new();
                    while *i < endbit {
                        insts.push(parse(b, i));
                    }
                    insts
                }
                _ => {
                    let id = calculate(b, i, 11);
                    (0..id).map(|_| parse(b, i)).collect()
                }
            };
            Instruction::Operator(version, id_, insts)
        }
    }
}

fn sum_versions(instruction: &Instruction) -> usize {
    match instruction {
        Instruction::Literal(version, _) => *version as usize,
        Instruction::Operator(version, _, instructions) => {
            *version as usize + instructions.iter().map(sum_versions).sum::<usize>()
        }
    }
}

fn calculate_value(instructions: &Instruction) -> usize {
    match instructions {
        Instruction::Literal(_, val) => *val as usize,
        Instruction::Operator(_, id, instructionss) => match id {
            0 => instructionss.iter().map(calculate_value).sum(),
            1 => instructionss.iter().map(calculate_value).product(),
            2 => instructionss.iter().map(calculate_value).min().unwrap(),
            3 => instructionss.iter().map(calculate_value).max().unwrap(),
            5 => (calculate_value(&instructionss[0]) > calculate_value(&instructionss[1])) as usize,
            6 => (calculate_value(&instructionss[0]) < calculate_value(&instructionss[1])) as usize,
            7 => {
                (calculate_value(&instructionss[0]) == calculate_value(&instructionss[1])) as usize
            }
            _ => unreachable!(),
        },
    }
}

impl AoCDay for Code {
    fn part1(&self, _input: &mut dyn std::io::Read, _extra_argss: &[String]) -> String {
        let data = load_file(_input);
        let lines: Vec<String> = parse_lines::<String>(&data).collect();
        let cords_data: Vec<String> = lines[0].chars().map(to_binary).collect();
        let concat: Vec<u8> = cords_data
            .iter()
            .flat_map(|bits| bits.bytes().map(|b| b - b'0'))
            .collect();

        let instructions = parse(&concat, &mut 0);

        let count = sum_versions(&instructions);
        count.to_string()
    }
    fn part2(&self, _input: &mut dyn std::io::Read, _extra_args: &[String]) -> String {
        let data = load_file(_input);
        let lines: Vec<String> = parse_lines::<String>(&data).collect();
        let cords_data: Vec<String> = lines[0].chars().map(to_binary).collect();
        let concat: Vec<u8> = cords_data
            .iter()
            .flat_map(|bits| bits.bytes().map(|b| b - b'0'))
            .collect();

        let instructions = parse(&concat, &mut 0);
        println!("{:?}", instructions);

        let count = calculate_value(&instructions);
        count.to_string()
    }
}
