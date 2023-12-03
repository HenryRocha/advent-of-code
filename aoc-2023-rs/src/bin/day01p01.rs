use clap::Parser;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the problem's input file.
    #[arg(short, long, value_name = "FILE")]
    input: PathBuf,

    /// Turn debugging information on.
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

fn process_line(line: String) -> u32 {
    let mut number = 0;

    for c in line.chars() {
        if c.is_numeric() {
            number += c.to_digit(10).expect("Failed to parse digit to u32.") * 10;
            break;
        }
    }

    for c in line.chars().rev() {
        if c.is_numeric() {
            number += c.to_digit(10).expect("Failed to parse digit to u32.");
            break;
        }
    }

    number
}

fn process_input(lines: Lines<BufReader<File>>) -> u32 {
    let mut result: u32 = 0;

    for line in lines {
        match line {
            Ok(line) => {
                result += process_line(line);
            }
            Err(_) => {
                println!("Failed to read input file line.");
            }
        }
    }

    result
}

fn main() {
    let cli = Cli::parse();

    let file = File::open(cli.input).expect("Failed to read input file.");
    let reader = BufReader::new(file);
    println!("result = {}", process_input(reader.lines()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calibrate() {
        let file_path = "inputs\\day01p01-example.txt";
        let file = File::open(file_path).expect("Failed to read input file.");
        let reader = BufReader::new(file);
        assert_eq!(process_input(reader.lines()), 142);
    }
}
