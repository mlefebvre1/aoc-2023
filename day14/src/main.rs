use std::str::FromStr;

use common::Platform;
use util::Cli;

mod common;

fn main() {
    let cli = Cli::get();
    println!("part1={}", part1(&cli.file));
    println!("part2={}", part2(&cli.file));
}

fn part1(file: &str) -> String {
    let data = std::fs::read_to_string(file).unwrap();
    let mut platform = Platform::from_str(&data).unwrap();
    platform.tilt_north();
    let ans = platform.calculate_score();
    ans.to_string()
}

fn part2(file: &str) -> String {
    let data = std::fs::read_to_string(file).unwrap();
    let mut platform = Platform::from_str(&data).unwrap();
    // Through observation after a while (around 500), a pattern of 9 numbers repeats
    //505 = 99641
    //506 = 99630
    //507 = 99623
    //508 = 99618
    //509 = 99621
    //510 = 99625
    //511 = 99652
    //512 = 99654
    //513 = 99646
    //514 = 99641 <-
    // The nice thing about 505 is that (1000000000 - 505) % 9 == 0
    // so we can just use the value that 1000 spits to solve the puzzle!
    for _ in 0..505 {
        platform.run_cycle();
    }

    let ans = platform.calculate_score();
    ans.to_string()
}
