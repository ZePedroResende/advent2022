use crate::{load_file, AoCDay};
pub struct Code;
impl AoCDay for Code {
    fn part1(&self, _input: &mut dyn std::io::Read, _extra_argss: &[String]) -> String {
        let data = load_file(_input);
        let numbers: Vec<usize> = data
            .lines().next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<usize>().expect("Not a number"))
            .collect();

        let mut population = [0; 9];
        for number in numbers {
            population[number] += 1;
        }

        for _ in 0..80 {
            population.rotate_left(1);
            population[6] += population[8];
        }

        population.iter().sum::<usize>().to_string()
    }

    fn part2(&self, _input: &mut dyn std::io::Read, _extra_args: &[String]) -> String {
        let data = load_file(_input);
        let numbers: Vec<usize> = data
            .lines().next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<usize>().expect("Not a number"))
            .collect();

        let mut population = [0; 9];
        for number in numbers {
            population[number] += 1;
        }

        for _ in 0..256 {
            population.rotate_left(1);
            population[6] += population[8];
        }

        population.iter().sum::<usize>().to_string()
    }
}
