use crate::day01::cli::CliArgs;
use crate::shared::enums::Part;
use crate::utils::file::read_file_buffered;

use std::fs::File;
use std::io::{BufRead, BufReader};
use tracing::{debug, info};

pub fn solve(args: &CliArgs) -> u32 {
    debug!("Running solver for day 01. input = {:?}", args);

    debug!("Reading file as buffer: {:?}", args.calibration);
    let reader = read_file_buffered(&args.calibration);

    let result;
    match args.part {
        Part::One => {
            info!("Solving for part one.");
            result = part_one(reader);
        }
        Part::Two => {
            info!("Solving for part two.");
            result = part_two(reader);
        }
    }

    info!("Result = {}", result);
    result
}

fn part_one(reader: BufReader<File>) -> u32 {
    let mut result: u32 = 0;

    for line in reader.lines() {
        result += calibrate_line(line.expect("Should be able to read file line"));
    }

    result
}

fn part_two(reader: BufReader<File>) -> u32 {
    let mut result: u32 = 0;

    for line in reader.lines() {
        result += calibrate_line(preprocess_calibration_line(
            line.expect("Should be able to read file line"),
        ));
    }

    result
}

fn preprocess_calibration_line(line: String) -> String {
    line.replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
        .replace("zero", "zero0zero")
}

fn calibrate_line(line: String) -> u32 {
    let mut result = 0;

    for c in line.chars() {
        if c.is_numeric() {
            result += parse_digit(c) * 10;
            break;
        }
    }

    for c in line.chars().rev() {
        if c.is_numeric() {
            result += parse_digit(c);
            break;
        }
    }

    result
}

fn parse_digit(c: char) -> u32 {
    c.to_digit(10).expect("Failed to parse digit to u32")
}
