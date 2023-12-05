#![allow(dead_code)]

use clap::Parser;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
use std::path::PathBuf;

#[derive(Debug)]
struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}

impl Bag {
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game<'a> {
    id: u32,
    bag: &'a Bag,
    rounds: Vec<Round>,
}

impl<'a> Game<'a> {
    fn is_valid(&self) -> bool {
        for round in self.rounds.iter() {
            if round.red > self.bag.red
                || round.green > self.bag.green
                || round.blue > self.bag.blue
            {
                return false;
            }
        }

        return true;
    }

    fn min_bag(&self) -> Bag {
        let mut min_bag = Bag {
            red: 0,
            green: 0,
            blue: 0,
        };

        self.rounds
            .iter()
            .map(|round| {
                if round.red > min_bag.red {
                    min_bag.red = round.red;
                }

                if round.green > min_bag.green {
                    min_bag.green = round.green;
                }

                if round.blue > min_bag.blue {
                    min_bag.blue = round.blue;
                }
            })
            .for_each(drop);

        min_bag
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Number of red cubes in the bag.
    #[arg(short, long, value_name = "BAG")]
    red: u32,

    /// Number of green cubes in the bag.
    #[arg(short, long, value_name = "BAG")]
    green: u32,

    /// Number of blue cubes in the bag.
    #[arg(short, long, value_name = "BAG")]
    blue: u32,

    /// Path to the problem's games file.
    #[arg(short, long, value_name = "FILE")]
    games: PathBuf,

    /// Turn debugging information on.
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

fn parse_rounds(rounds_str: &str) -> Vec<Round> {
    rounds_str
        .split("; ")
        .map(|round_str| {
            let mut round = Round {
                red: 0,
                green: 0,
                blue: 0,
            };

            round_str
                .split(", ")
                .map(|cube_str| {
                    let cube_info: Vec<&str> = cube_str.split(" ").collect();

                    let cube_amount: u32 = cube_info[0]
                        .parse()
                        .expect("Cube amount is supposed to be a u32.");

                    match cube_info[1] {
                        "red" => round.red += cube_amount,
                        "green" => round.green += cube_amount,
                        "blue" => round.blue += cube_amount,
                        _ => {}
                    };
                })
                .for_each(drop);

            round
        })
        .collect::<Vec<Round>>()
}

fn parse_game_id(game_info: &str) -> u32 {
    game_info
        .replace("Game ", "")
        .parse::<u32>()
        .expect("Game ID is supposed to be a u32.")
}

fn parse_game(bag: &Bag, line: String) -> Game {
    let game: Vec<&str> = line.split(": ").collect();

    Game {
        id: parse_game_id(game[0]),
        bag: bag,
        rounds: parse_rounds(game[1]),
    }
}

fn process_input(bag: &Bag, lines: Lines<BufReader<File>>) -> u32 {
    let mut result: u32 = 0;

    for line in lines {
        let game = parse_game(bag, line.expect("Line should be readable."));
        println!("{:?}", game);

        result += game.min_bag().power();
    }

    result
}

fn main() {
    let cli = Cli::parse();

    let file = File::open(cli.games).expect("Failed to read games file.");
    let reader = BufReader::new(file);

    println!(
        "result = {}",
        process_input(
            &Bag {
                red: cli.red,
                green: cli.green,
                blue: cli.blue
            },
            reader.lines()
        )
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let file_path = "inputs\\day02p01-example.txt";
        let file = File::open(file_path).expect("Failed to read input file.");
        let reader = BufReader::new(file);

        assert_eq!(
            process_input(
                &Bag {
                    red: 12,
                    green: 13,
                    blue: 14
                },
                reader.lines()
            ),
            2286
        );
    }

    #[test]
    fn test_input() {
        let file_path = "inputs\\day02p01.txt";
        let file = File::open(file_path).expect("Failed to read input file.");
        let reader = BufReader::new(file);

        assert_eq!(
            process_input(
                &Bag {
                    red: 12,
                    green: 13,
                    blue: 14
                },
                reader.lines()
            ),
            56580
        );
    }
}
