use itertools::Itertools;

fn main() {
    let input = include_str!("../inputs/day4.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let input = parse_input(input);

    let mut sum = 0;
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            sum += count_xmas_in_all_directions(&input, x, y);
        }
    }
    sum
}

// returns num of XMAX
fn count_xmas_in_all_directions(input: &[[char; 140]; 140], x: usize, y: usize) -> u32 {
    const S: &str = "XMAS";

    let right = S.chars().enumerate().all(|(i, c)| {
        if let Some(line) = input.get(y) {
            if line.get(x + i) == Some(&c) {
                return true;
            }
        }
        false
    });

    let left = S.chars().enumerate().all(|(i, c)| {
        if let Some(line) = input.get(y) {
            let Some(x_i) = x.checked_sub(i) else {
                return false;
            };
            if line.get(x_i) == Some(&c) {
                return true;
            }
        }
        false
    });

    let up = S.chars().enumerate().all(|(i, c)| {
        let Some(y_i) = y.checked_sub(i) else {
            return false;
        };
        if let Some(line) = input.get(y_i) {
            if line.get(x) == Some(&c) {
                return true;
            }
        }
        false
    });

    let down = S.chars().enumerate().all(|(i, c)| {
        if let Some(line) = input.get(y + i) {
            if line.get(x) == Some(&c) {
                return true;
            }
        }
        false
    });

    let top_left = S.chars().enumerate().all(|(i, c)| {
        let Some(y_i) = y.checked_sub(i) else {
            return false;
        };
        let Some(x_i) = x.checked_sub(i) else {
            return false;
        };
        if let Some(line) = input.get(y_i) {
            if line.get(x_i) == Some(&c) {
                return true;
            }
        }
        false
    });
    let top_right = S.chars().enumerate().all(|(i, c)| {
        let Some(y_i) = y.checked_sub(i) else {
            return false;
        };
        if let Some(line) = input.get(y_i) {
            if line.get(x + i) == Some(&c) {
                return true;
            }
        }
        false
    });
    let bottom_left = S.chars().enumerate().all(|(i, c)| {
        let Some(x_i) = x.checked_sub(i) else {
            return false;
        };
        if let Some(line) = input.get(y + i) {
            if line.get(x_i) == Some(&c) {
                return true;
            }
        }
        false
    });
    let bottom_right = S.chars().enumerate().all(|(i, c)| {
        if let Some(line) = input.get(y + i) {
            if line.get(x + i) == Some(&c) {
                return true;
            }
        }
        false
    });

    [
        right,
        left,
        up,
        down,
        top_left,
        top_right,
        bottom_left,
        bottom_right,
    ]
    .into_iter()
    .map(|x| x as u32)
    .sum()
}

fn part2(input: &str) -> u32 {
    let input = parse_input(input);

    let mut sum = 0;
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] != 'A' {
                continue;
            }

            let Some(x_i) = x.checked_sub(1) else {
                continue;
            };
            let Some(y_i) = y.checked_sub(1) else {
                continue;
            };

            // left to right
            if input.get(y_i).and_then(|line| line.get(x_i)) == Some(&'M') {
                if input.get(y + 1).and_then(|line| line.get(x_i)) == Some(&'M') {
                    if input.get(y_i).and_then(|line| line.get(x + 1)) == Some(&'S') {
                        if input.get(y + 1).and_then(|line| line.get(x + 1)) == Some(&'S') {
                            sum += 1;
                            continue;
                        }
                    }
                }
            }

            // right to left
            if input.get(y_i).and_then(|line| line.get(x_i)) == Some(&'S') {
                if input.get(y + 1).and_then(|line| line.get(x_i)) == Some(&'S') {
                    if input.get(y_i).and_then(|line| line.get(x + 1)) == Some(&'M') {
                        if input.get(y + 1).and_then(|line| line.get(x + 1)) == Some(&'M') {
                            sum += 1;
                            continue;
                        }
                    }
                }
            }

            // top to bottom
            if input.get(y_i).and_then(|line| line.get(x_i)) == Some(&'M') {
                if input.get(y + 1).and_then(|line| line.get(x_i)) == Some(&'S') {
                    if input.get(y_i).and_then(|line| line.get(x + 1)) == Some(&'M') {
                        if input.get(y + 1).and_then(|line| line.get(x + 1)) == Some(&'S') {
                            sum += 1;
                            continue;
                        }
                    }
                }
            }

            // top to bottom
            if input.get(y_i).and_then(|line| line.get(x_i)) == Some(&'S') {
                if input.get(y + 1).and_then(|line| line.get(x_i)) == Some(&'M') {
                    if input.get(y_i).and_then(|line| line.get(x + 1)) == Some(&'S') {
                        if input.get(y + 1).and_then(|line| line.get(x + 1)) == Some(&'M') {
                            sum += 1;
                        }
                    }
                }
            }
        }
    }
    sum
}

// row major
fn parse_input(input: &str) -> [[char; 140]; 140] {
    input
        .lines()
        .map(|line| {
            line.chars()
                .collect_vec()
                .try_into()
                .expect("dimensions of 140x140")
        })
        .collect_vec()
        .try_into()
        .expect("dimensions of 140x140")
}
