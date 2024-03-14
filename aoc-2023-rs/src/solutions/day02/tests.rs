#[cfg(test)]
use crate::day02;
use crate::shared::enums::Part;

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[test]
fn part_one_example() {
    let cli_args = day02::cli::CliArgs {
        games: PathBuf::from("inputs\\day02p01-example.txt"),
        red: 12,
        green: 13,
        blue: 14,
        part: Part::One,
    };
    assert_eq!(day02::solver::solve(&cli_args), 142);
}

#[test]
fn part_one_final() {
    let cli_args = day02::cli::CliArgs {
        games: PathBuf::from("inputs\\day02p01.txt"),
        red: 12,
        green: 13,
        blue: 14,
        part: Part::One,
    };
    assert_eq!(day02::solver::solve(&cli_args), 53974);
}

#[test]
fn part_two_example() {
    let cli_args = day02::cli::CliArgs {
        games: PathBuf::from("inputs\\day02p02-example.txt"),
        red: 12,
        green: 13,
        blue: 14,
        part: Part::Two,
    };
    assert_eq!(day02::solver::solve(&cli_args), 281);
}

#[test]
fn part_two_final() {
    let cli_args = day02::cli::CliArgs {
        games: PathBuf::from("inputs\\day02p02.txt"),
        red: 12,
        green: 13,
        blue: 14,
        part: Part::Two,
    };
    assert_eq!(day02::solver::solve(&cli_args), 52840);
}
