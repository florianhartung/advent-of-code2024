use itertools::Itertools;

fn main() {
    let input = include_str!("../inputs/day1.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let (mut left_values, mut right_values): (Vec<u32>, Vec<u32>) = parse_input(input).unzip();

    left_values.sort();
    right_values.sort();

    left_values
        .into_iter()
        .zip(right_values.into_iter())
        .map(|(left, right)| left.abs_diff(right))
        .sum()
}

fn part2(input: &str) -> u32 {
    let rows = parse_input(input).collect_vec();
    let frequencies = rows.iter().counts_by(|&(_, right)| right);

    rows.into_iter()
        .map(|(left, _)| left * frequencies.get(&left).copied().unwrap_or(0) as u32)
        .sum()
}

fn parse_input(input: &str) -> impl Iterator<Item = (u32, u32)> + '_ {
    input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(left, right)| (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap()))
}
