use crate::shared::enums::Part;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct CliArgs {
    /// Path to the problem's input file.
    pub calibration: PathBuf,

    /// Which part to solve for.
    #[clap(short, long, value_enum, default_value = "one")]
    pub part: Part,
}
