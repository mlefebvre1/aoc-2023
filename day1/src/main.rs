fn main() {
    let puzzle_input = util::fetch_puzzle_input(1).unwrap();
    println!("part1={}", part1(&puzzle_input));
    println!("part2={}", part2(&puzzle_input));
}

fn part1(data: &str) -> String {
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

fn part2(data: &str) -> String {
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
