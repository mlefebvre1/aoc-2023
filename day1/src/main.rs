fn main() {
    println!("part1={}", part1());
    println!("part2={}", part2());
}

fn part1() -> String {
    let data = std::fs::read_to_string("day1/data/day1.txt").unwrap();
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

fn part2() -> String {
    let data = std::fs::read_to_string("day1/data/day1.txt").unwrap();
    let ans: u32 = data
        .lines()
        .map(|line| {
            let line_replaced = replace_with_digit(line.to_string());
            let digits: Vec<u32> = line_replaced
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect();
            let twodigit = digits.last().unwrap() + digits.first().unwrap() * 10;
            twodigit
        })
        .sum();

    ans.to_string()
}

fn replace_with_digit(mut line: String) -> String {
    const DIGITS_LETTERS: [(&str, &str); 9] = [
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "4"),
        ("five", "5e"),
        ("six", "6"),
        ("seven", "7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ];

    for (letters, digit) in DIGITS_LETTERS {
        line = line.replace(letters, digit);
    }
    line
}
