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

    let data = std::fs::read_to_string("day1/data/day1.txt").unwrap();
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
