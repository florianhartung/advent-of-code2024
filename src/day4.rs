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

    let get_val = |dx: isize, dy: isize| {
        let x = x.checked_add_signed(dx)?;
        let y = y.checked_add_signed(dy)?;
        input.get(y).and_then(|line| line.get(x))
    };

    let chars = S.chars().enumerate().map(|(i, c)| (i as isize, c));

    let right = chars.clone().all(|(i, c)| get_val(i, 0) == Some(&c));
    let left = chars.clone().all(|(i, c)| get_val(-i, 0) == Some(&c));
    let up = chars.clone().all(|(i, c)| get_val(0, -i) == Some(&c));
    let down = chars.clone().all(|(i, c)| get_val(0, i) == Some(&c));
    let top_left = chars.clone().all(|(i, c)| get_val(-i, -i) == Some(&c));
    let top_right = chars.clone().all(|(i, c)| get_val(i, -i) == Some(&c));
    let bottom_left = chars.clone().all(|(i, c)| get_val(-i, i) == Some(&c));
    let bottom_right = chars.clone().all(|(i, c)| get_val(i, i) == Some(&c));

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
            let get_val = |dx: isize, dy: isize| {
                let x = x.checked_add_signed(dx)?;
                let y = y.checked_add_signed(dy)?;
                input.get(y).and_then(|line| line.get(x))
            };

            let Some(&top_left) = get_val(-1, -1) else {
                continue;
            };
            let Some(&top_right) = get_val(1, -1) else {
                continue;
            };
            let Some(&bottom_left) = get_val(-1, 1) else {
                continue;
            };
            let Some(&bottom_right) = get_val(1, 1) else {
                continue;
            };

            // middle is always A
            if get_val(0, 0) != Some(&'A') {
                continue;
            }

            // left to right
            if top_left == 'M' && bottom_left == 'M' && top_right == 'S' && bottom_right == 'S' {
                sum += 1;
            }

            // right to left
            if top_left == 'S' && bottom_left == 'S' && top_right == 'M' && bottom_right == 'M' {
                sum += 1;
            }

            // top to bottom
            if top_left == 'M' && bottom_left == 'S' && top_right == 'M' && bottom_right == 'S' {
                sum += 1;
            }

            // bottom to top
            if top_left == 'S' && bottom_left == 'M' && top_right == 'S' && bottom_right == 'M' {
                sum += 1;
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