use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let input = include_str!("../inputs/day1.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let (mut left_values, mut right_values): (Vec<u32>, Vec<u32>) = split_columns(input).unzip();

    left_values.sort();
    right_values.sort();

    left_values
        .into_iter()
        .zip(right_values.into_iter())
        .map(|(left, right)| left.abs_diff(right))
        .sum()
}

fn part2(input: &str) -> u32 {
    let mut frequencies = HashMap::<u32, u32>::new();
    let rows = split_columns(input).collect_vec();

    for &(_, right) in &rows {
        frequencies
            .entry(right)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    rows.into_iter()
        .map(|(left, _)| left * frequencies.get(&left).copied().unwrap_or(0))
        .sum()
}

fn split_columns(input: &str) -> impl Iterator<Item = (u32, u32)> + '_ {
    input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(left, right)| (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap()))
}
