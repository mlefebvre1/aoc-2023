use util::Cli;

mod common;
mod part1;

fn main() {
    let cli = Cli::get();
    println!("part1={}", part1::run(&cli.file));
}
