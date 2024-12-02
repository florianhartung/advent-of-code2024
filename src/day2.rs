use itertools::Itertools;

fn main() {
    let input = include_str!("../inputs/day2.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    parse_input(input)
        .map(|report| is_report_safe(report) as u32)
        .sum()
}

fn part2(input: &str) -> u32 {
    parse_input(input)
        .map(|report| {
            let report = report.collect_vec();
            for i in 0..report.len() {
                let mut cloned = report.clone();
                cloned.remove(i);
                if is_report_safe(cloned.iter().copied()) {
                    return 1;
                }
            }
            0
        })
        .sum()
}

fn parse_input(input: &str) -> impl Iterator<Item = impl Iterator<Item = u32> + '_> {
    input.lines().map(|report| {
        report
            .split_ascii_whitespace()
            .map(|level| level.parse::<u32>().unwrap())
    })
}

fn is_report_safe(report: impl Iterator<Item = u32>) -> bool {
    let mut is_increasing = true;
    let mut is_decreasing = true;
    let mut large_delta_found = false;

    report.tuple_windows().for_each(|(previous_level, level)| {
        if previous_level < level {
            is_decreasing = false;
        } else if previous_level > level {
            is_increasing = false;
        }

        if !((1..=3).contains(&previous_level.abs_diff(level))) {
            large_delta_found = true;
        }
    });

    (is_increasing || is_decreasing) && !large_delta_found
}
