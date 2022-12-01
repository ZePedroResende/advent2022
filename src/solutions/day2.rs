use crate::{load_file, parse_lines, AoCDay};
use std::str::FromStr;
use std::result::Result;
pub struct Code;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Forward,
    Down,
    Up,
}
#[derive(Debug, Clone, Copy)]
struct Instruction {
    direction: Direction,
    amount: u64,
}

impl FromStr for Direction {
    type Err = ();
    fn from_str(direction: &str) -> Result<Self,()> {
        match direction {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(()),
        }
    }
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(instruction: &str) -> Result<Self,()> {
        let (direction_str, amount_str) = instruction.split_once(" ").unwrap();
        let direction = direction_str
            .parse::<Direction>()
            .expect("Couldn't parse direction");
        let amount = amount_str.parse::<u64>().expect("Couldn't parse amount");
        Ok(Instruction { direction, amount })
    }
}

impl Instruction {

    fn sum(self,(a,b,c): (u64,u64,u64)) -> (u64,u64,u64){
        match self.direction {
            Direction::Forward=>(a+self.amount,b+ (self.amount * c),c) ,
            Direction::Down=> (a,b,c+self.amount) ,
            Direction::Up=> (a,b,c-self.amount) ,
        }
    }
}

impl AoCDay for Code {
    fn part1(&self, _input: &mut dyn std::io::Read, _extra_argss: &[String]) -> String {
        let data = load_file(_input);
        let instructions: Vec<Instruction> = parse_lines::<Instruction>(&data).collect();

        let (horizontal,_depth, aim) = instructions.iter().fold((0u64,0u64,0u64), |sum, val| val.sum(sum));
        (horizontal*aim).to_string()
    }

    fn part2(&self, _input: &mut dyn std::io::Read, _extra_args: &[String]) -> String {
        let data = load_file(_input);
        let instructions: Vec<Instruction> = parse_lines::<Instruction>(&data).collect();

        let (horizontal,depth, _aim) = instructions.iter().fold((0u64,0u64,0u64), |sum, val| val.sum(sum));
        (horizontal*depth).to_string()
    }
}
