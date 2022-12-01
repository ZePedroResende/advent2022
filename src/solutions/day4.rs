use crate::{load_file, AoCDay};
use std::str::FromStr;
pub struct Code;

#[derive(Debug, Clone, Copy)]
struct Tile {
    number: u64,
    marked: bool,
}

#[derive(Debug, Clone)]
struct Card {
    tiles: Vec<Vec<Tile>>,
}

#[derive(Debug, Clone)]
struct Cards {
    cards: Vec<Card>,
}

impl From<u64> for Tile {
    fn from(number: u64) -> Self {
        Self {
            number,
            marked: false,
        }
    }
}

impl From<Vec<Card>> for Cards {
    fn from(cards: Vec<Card>) -> Self {
        Self { cards }
    }
}

impl FromStr for Card {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles: Vec<Vec<Tile>> = s
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(|line| {
                line.split_whitespace()
                    .map(|n| n.parse::<u64>().expect("Should be parasable number"))
                    .map(From::from)
                    .collect()
            })
            .collect();

        assert!(tiles.len() == 5 && tiles.iter().all(|card| card.len() == 5));

        Ok(Self { tiles })
    }
}

impl Card {
    fn unmarked(&self) -> u64 {
        self.tiles
            .iter()
            .flat_map(|x| x.iter())
            .filter_map(|e| (!e.marked).then(|| e.number))
            .sum::<u64>()
    }

    fn mark(&mut self, number: u64) {
        for row in &mut self.tiles {
            for tile in row {
                if !tile.marked && (tile.number == number) {
                    tile.marked = true;
                }
            }
        }
    }

    fn won(&self) -> bool {
        (0..self.tiles.len()).any(|row| self.tiles[row].iter().all(|e| e.marked))
            || (0..self.tiles.len()).any(|col| self.tiles.iter().all(|row| row[col].marked))
    }
}

impl Cards {
    fn solve(&mut self, plays: Vec<u64>) -> (Card, u64) {
        for play in plays.iter() {
            for card in &mut *self.cards {
                if card.clone().won() {
                    continue;
                }
                card.mark(*play);

                if card.won() {
                    return (card.clone(), *play);
                }
            }
        }
        panic!("error");
    }

    fn solve_last(&mut self, plays: Vec<u64>) -> (Card, u64) {
        let mut cards_finished = 0;
        let len = self.cards.len();

        for play in plays.iter() {
            for card in &mut *self.cards {
                if card.won() {
                    continue;
                }
                card.mark(*play);

                if card.won() {
                    cards_finished += 1;
                    if cards_finished == len {
                        return (card.clone(), *play);
                    }
                }
            }
        }
        panic!("error");
    }
}

impl AoCDay for Code {
    fn part1(&self, _input: &mut dyn std::io::Read, _extra_argss: &[String]) -> String {
        let data = load_file(_input);
        let plays: Vec<u64> = data
            .lines().next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<u64>().expect("Not a number"))
            .collect();

        let mut cards: Cards = Cards::from(
            data.split("\n\n")
                .skip(1)
                .map(FromStr::from_str)
                .collect::<Result<Vec<Card>, String>>()
                .expect("Not cards"),
        );

        let (board, last) = cards.solve(plays);
        (board.unmarked() * last).to_string()
    }
    fn part2(&self, _input: &mut dyn std::io::Read, _extra_args: &[String]) -> String {
        let data = load_file(_input);
        let plays: Vec<u64> = data
            .lines().next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<u64>().expect("Not a number"))
            .collect();

        let mut cards: Cards = Cards::from(
            data.split("\n\n")
                .skip(1)
                .map(FromStr::from_str)
                .collect::<Result<Vec<_>, String>>()
                .expect("Not cards"),
        );

        let (board, last) = cards.solve_last(plays);
        (board.unmarked() * last).to_string()
    }
}
