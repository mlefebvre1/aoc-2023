fn main() {
    let cli = util::Cli::get();
    println!("part1={}", part1(&cli.file));
    println!("part2={}", part2(&cli.file));
}

fn part1(file: &str) -> String {
    let data = std::fs::read_to_string(file).unwrap();
    let ans: u32 = data
        .lines()
        .map(|line| {
            let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
            let twodigit = digits.last().unwrap() + digits.first().unwrap() * 10;
            twodigit
        })
        .sum();

    ans.to_string()
}

fn part2(file: &str) -> String {
    let replace_with_digit = |line: &str| {
        line.replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "4")
            .replace("five", "5e")
            .replace("six", "6")
            .replace("seven", "7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e")
    };

    let data = std::fs::read_to_string(file).unwrap();
    let ans: u32 = data
        .lines()
        .map(|line| {
            let digits: Vec<u32> = replace_with_digit(line)
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect();
            let twodigit = digits.last().unwrap() + digits.first().unwrap() * 10;
            twodigit
        })
        .sum();

    ans.to_string()
}
