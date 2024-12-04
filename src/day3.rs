use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let input = include_str!("../inputs/day3.txt").lines().join("");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> u32 {
    parse_input(&input)
        .map(|(first, second)| first * second)
        .sum()
}

fn part2(input: &str) -> u32 {
    let re = Regex::new(r"don't\(\).*?do\(\)").unwrap();
    let input = re.replace_all(&input, "").to_owned();

    part1(input.as_ref())
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"mul\((\d{1, 3}),(\d{1, 3})\)").unwrap();
}

fn parse_input(input: &str) -> impl Iterator<Item = (u32, u32)> + '_ {
    RE.captures_iter(&input).map(|capture| {
        let (_, [first, second]) = capture.extract();
        let first: u32 = first.parse().unwrap();
        let second: u32 = second.parse().unwrap();

        (first, second)
    })
}
