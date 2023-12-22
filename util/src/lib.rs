mod cli;

pub mod grid;

pub use cli::Cli;

#[macro_export]
macro_rules! run {
    () => {
        use util::Cli;
        let cli = Cli::get();
        println!("part1={}", part1(&cli.file));
        println!("part2={}", part2(&cli.file));
    };
}
