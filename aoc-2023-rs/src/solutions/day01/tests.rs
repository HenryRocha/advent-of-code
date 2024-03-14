#[cfg(test)]
use crate::day01;
use crate::shared::enums::Part;

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[test]
fn part_one_example() {
    let cli_args = day01::cli::CliArgs {
        calibration: PathBuf::from("inputs\\day01p01-example.txt"),
        part: Part::One,
    };
    assert_eq!(day01::solver::solve(&cli_args), 142);
}

#[test]
fn part_one_final() {
    let cli_args = day01::cli::CliArgs {
        calibration: PathBuf::from("inputs\\day01p01.txt"),
        part: Part::One,
    };
    assert_eq!(day01::solver::solve(&cli_args), 53974);
}

#[test]
fn part_two_example() {
    let cli_args = day01::cli::CliArgs {
        calibration: PathBuf::from("inputs\\day01p02-example.txt"),
        part: Part::Two,
    };
    assert_eq!(day01::solver::solve(&cli_args), 281);
}

#[test]
fn part_two_final() {
    let cli_args = day01::cli::CliArgs {
        calibration: PathBuf::from("inputs\\day01p02.txt"),
        part: Part::Two,
    };
    assert_eq!(day01::solver::solve(&cli_args), 52840);
}
