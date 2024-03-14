use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Debug, Clone)]
pub enum Part {
    One = 1,
    Two = 2,
}
