use util::Cli;

mod common;
mod part1;
mod part2;

fn main() {
    let cli = Cli::get();
    println!("part1={}", part1::run(&cli.file));
    println!("part2={}", part2::run(&cli.file));
}
