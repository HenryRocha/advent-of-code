use crate::day02::cli::CliArgs;
use crate::shared::enums::Part;
use crate::utils::file::read_file_buffered;

use std::fs::File;
use std::io::{BufRead, BufReader};
use tracing::{debug, info};

pub fn solve(args: &CliArgs) -> u32 {
    debug!("Running solver for day 01. input = {:?}", args);

    debug!("Reading file as buffer: {:?}", args.games);
    let reader = read_file_buffered(&args.games);

    let result = 0;
    match args.part {
        Part::One => {
            info!("Solving for part one.");
        }
        Part::Two => {
            info!("Solving for part two.");
        }
    }

    info!("Result = {}", result);
    result
}
