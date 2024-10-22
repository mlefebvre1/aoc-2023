use anyhow::Result;

pub fn fetch_puzzle_input(puzzle_number: u8) -> Result<String> {
    let aoc_token = std::env::var("AOC_TOKEN")?;

    let client = reqwest::blocking::Client::new();

    let resp = client
        .get(format!(
            "https://adventofcode.com/2023/day/{}/input",
            puzzle_number
        ))
        .header("Cookie", aoc_token)
        .send()?;

    Ok(resp.text()?)
}
