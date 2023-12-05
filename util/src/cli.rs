use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    pub file: String,
}

impl Cli {
    pub fn get() -> Self {
        Self::parse()
    }
}
