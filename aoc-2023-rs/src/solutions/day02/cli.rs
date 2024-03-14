use crate::shared::enums::Part;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct CliArgs {
    /// Path to the problem's input file.
    pub games: PathBuf,

    /// Number of red cubes in the bag.
    #[clap(short, long, default_value_t = 12)]
    pub red: u32,

    /// Number of blue cubes in the bag.
    #[clap(short, long, default_value_t = 13)]
    pub blue: u32,

    /// Number of green cubes in the bag.
    #[clap(short, long, default_value_t = 14)]
    pub green: u32,

    /// Which part to solve for.
    #[clap(short, long, value_enum, default_value = "one")]
    pub part: Part,
}
