mod cli;

pub mod grid;
mod puzzle_input;

pub use cli::Cli;
pub use puzzle_input::fetch_puzzle_input;

#[macro_export]
macro_rules! run {
    () => {
        use util::Cli;
        let cli = Cli::get();
        println!("part1={}", part1(&cli.file));
        println!("part2={}", part2(&cli.file));
    };
}
