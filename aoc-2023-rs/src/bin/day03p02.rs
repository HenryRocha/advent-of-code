#![allow(dead_code)]

use clap::Parser;
use std::fmt;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
use std::path::PathBuf;

#[derive(PartialEq, Debug)]
enum Direction {
    Left,
    Right,
    LeftAndRight,
}

#[derive(Default)]
struct Schematic {
    width: usize,
    height: usize,
    contents: Vec<Vec<char>>,
}

impl Schematic {
    fn scan_area_for_symbol(&self, i: usize, j: usize) -> bool {
        // (i - 1, j - 1) | (i - 1, j) | (i - 1, j + 1)
        // (i    , j - 1) |            | (i    , j + 1)
        // (i + 1, j - 1) | (i + 1, j) | (i + 1, j + 1)

        // Top left.
        if i > 0 && j > 0 {
            if self.is_symbol(i - 1, j - 1) {
                return true;
            };
        }

        // Top.
        if i > 0 {
            if self.is_symbol(i - 1, j) {
                return true;
            };
        }

        // Top right.
        if i > 0 && j < self.width - 1 {
            if self.is_symbol(i - 1, j + 1) {
                return true;
            };
        }

        // Left.
        if j > 0 {
            if self.is_symbol(i, j - 1) {
                return true;
            };
        }

        // Right.
        if j < self.width - 1 {
            if self.is_symbol(i, j + 1) {
                return true;
            };
        }

        // Bottom left.
        if i < self.height - 1 && j > 0 {
            if self.is_symbol(i + 1, j - 1) {
                return true;
            };
        }

        // Bottom.
        if i < self.height - 1 {
            if self.is_symbol(i + 1, j) {
                return true;
            };
        }

        // Bottom right.
        if i < self.height - 1 && j < self.width - 1 {
            if self.is_symbol(i + 1, j + 1) {
                return true;
            };
        }

        false
    }

    fn is_symbol(&self, i: usize, j: usize) -> bool {
        self.contents[i][j].is_ascii_punctuation() && self.contents[i][j] != '.'
    }

    fn scan_area_for_number(&self, i: usize, j: usize) -> Vec<u32> {
        // (i - 1, j - 1) | (i - 1, j) | (i - 1, j + 1)
        // (i    , j - 1) |            | (i    , j + 1)
        // (i + 1, j - 1) | (i + 1, j) | (i + 1, j + 1)
        let mut numbers = vec![];

        if i > 0 {
            // Top.
            if self.is_number(i - 1, j) {
                // Start number building from the top, going left and right.
                println!("Building T number.");
                numbers.push(self.build_number(i - 1, j, Direction::LeftAndRight));
            } else {
                // Top left.
                if j > 0 {
                    if self.is_number(i - 1, j - 1) {
                        // Start number building from the top left, going left.
                        println!("Building TL number.");
                        numbers.push(self.build_number(i - 1, j - 1, Direction::Left));
                    }
                }

                // Top right.
                if j < self.width - 1 {
                    if self.is_number(i - 1, j + 1) {
                        // Start number building from the top right, going right.
                        println!("Building TR number.");
                        numbers.push(self.build_number(i - 1, j + 1, Direction::Right));
                    }
                }
            }
        }

        // Left.
        if j > 0 {
            if self.is_number(i, j - 1) {
                // Start number building from the left, going left.
                println!("Building L number.");
                numbers.push(self.build_number(i, j - 1, Direction::Left));
            };
        }

        // Right.
        if j < self.width - 1 {
            if self.is_number(i, j + 1) {
                // Start number building from the right, going right.
                println!("Building R number.");
                numbers.push(self.build_number(i, j + 1, Direction::Right));
            };
        }

        if i < self.height - 1 {
            // Bottom.
            if self.is_number(i + 1, j) {
                // Start number building from the bottom, going left and right.
                println!("Building B number.");
                numbers.push(self.build_number(i + 1, j, Direction::LeftAndRight));
            } else {
                // Bottom left.
                if j > 0 {
                    if self.is_number(i + 1, j - 1) {
                        // Start number building from the bottom left, going left.
                        println!("Building BL number.");
                        numbers.push(self.build_number(i + 1, j - 1, Direction::Left));
                    };
                }

                // Bottom right.
                if j < self.width - 1 {
                    if self.is_number(i + 1, j + 1) {
                        // Start number building from the bottom right, going right.
                        println!("Building BR number.");
                        numbers.push(self.build_number(i + 1, j + 1, Direction::Right));
                    };
                }
            }
        }

        numbers
    }

    fn is_number(&self, i: usize, j: usize) -> bool {
        self.contents[i][j].is_numeric()
    }

    fn build_number(&self, i: usize, j: usize, dir: Direction) -> u32 {
        println!(
            "Building number for {} ({}, {}) going {:?}",
            self.contents[i][j], i, j, dir
        );
        let mut number = String::default();

        match dir {
            Direction::Left => {
                for c in self.contents[i][0..=j].iter().rev() {
                    if c.is_numeric() {
                        number.push(*c);
                    } else {
                        break;
                    }
                }

                // We need to reverse the string, so the digits are in the correct order.
                number = number.chars().rev().collect();
            }

            Direction::Right => {
                println!("Building number to the right.");
                for c in self.contents[i][j..=(self.width - 1)].iter() {
                    println!("Current number: {}, char {}", number, c);
                    if c.is_numeric() {
                        number.push(*c);
                    } else {
                        break;
                    }
                }
            }

            Direction::LeftAndRight => {
                for c in self.contents[i][0..=j].iter().rev() {
                    if c.is_numeric() {
                        number.push(*c);
                    } else {
                        break;
                    }
                }

                // We need to reverse the string, so the digits are in the correct order.
                number = number.chars().rev().collect();

                for c in self.contents[i][(j + 1)..=(self.width - 1)].iter() {
                    if c.is_numeric() {
                        number.push(*c);
                    } else {
                        break;
                    }
                }
            }
        }

        number.parse().expect("Gear number should be an u32.")
    }
}

impl fmt::Display for Schematic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Schematic: {} x {}", self.width, self.height).unwrap();

        for line in self.contents.iter() {
            for c in line {
                write!(f, "{}", c).unwrap();
            }

            write!(f, "\n").unwrap();
        }

        Ok(())
    }
}

#[derive(Default)]
struct Engine {
    schematic: Schematic,
    parts: Vec<u32>,
    gears: Vec<u32>,
}

impl Engine {
    fn extract_parts_from_schematic(&mut self) {
        let mut part_number = String::default();
        let mut has_symbol_around = false;

        for i in 0..self.schematic.height {
            for j in 0..self.schematic.width {
                let c = self.schematic.contents[i][j];

                if c.is_numeric() {
                    part_number.push(c);

                    // Scan the surroudings of this number, looking for symbols.
                    if !has_symbol_around {
                        has_symbol_around = self.schematic.scan_area_for_symbol(i, j);
                    }
                } else {
                    if !part_number.is_empty() {
                        if has_symbol_around {
                            self.parts.push(
                                part_number
                                    .parse::<u32>()
                                    .expect("Part number should be an u32."),
                            )
                        }

                        part_number.clear();
                        has_symbol_around = false;
                    }
                }
            }
        }
    }

    fn extract_gears_from_schematic(&mut self) {
        for i in 0..self.schematic.height {
            for j in 0..self.schematic.width {
                if self.schematic.contents[i][j] == '*' {
                    println!("Found * at ({}, {}). Scanning area for numbers...", i, j);

                    // Scan the surroundings for numbers.
                    let numbers = self.schematic.scan_area_for_number(i, j);
                    println!("{:?}", numbers);

                    if numbers.len() == 2 {
                        self.gears.push(numbers[0] * numbers[1]);
                    }
                }
            }
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the problem's input file.
    #[arg(short, long, value_name = "FILE")]
    input: PathBuf,
}

fn parse_schematic(lines: Lines<BufReader<File>>) -> Schematic {
    let mut schematic = Schematic::default();

    for (j, line) in lines.enumerate() {
        schematic.contents.push(vec![]);
        schematic.height += 1;

        for c in line.expect("Line should be readable.").chars() {
            schematic.contents[j].push(c);
        }
    }

    if schematic.height >= 1 {
        schematic.width = schematic.contents[0].len();
    }

    schematic
}

fn process_input(lines: Lines<BufReader<File>>) -> u32 {
    let mut engine = Engine::default();
    engine.schematic = parse_schematic(lines);
    println!("{}", engine.schematic);

    // Part 1.
    // engine.extract_parts_from_schematic();
    // engine.parts.iter().sum();

    // Part 2.
    engine.extract_gears_from_schematic();
    engine.gears.iter().sum()
}

fn main() {
    let cli = Cli::parse();

    let file = File::open(cli.input).expect("Failed to read games file.");
    let reader = BufReader::new(file);

    println!("result = {}", process_input(reader.lines()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let file_path = "inputs\\day03p01-example.txt";
        let file = File::open(file_path).expect("Failed to read input file.");
        let reader = BufReader::new(file);

        assert_eq!(process_input(reader.lines()), 467835);
    }

    #[test]
    fn test_input() {
        let file_path = "inputs\\day03p01.txt";
        let file = File::open(file_path).expect("Failed to read input file.");
        let reader = BufReader::new(file);

        assert_eq!(process_input(reader.lines()), 91622824);
    }
}
