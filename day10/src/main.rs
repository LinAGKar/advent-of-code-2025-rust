use std::{cmp::Reverse, collections::{BinaryHeap, HashSet}, io::Read};

use num_rational::Rational32;

fn get_lowest_button_presses(target: u32, buttons: &[u32], state: u32, index: usize, count: u32, best: &mut u32) {
    if count >= *best {
        // Nothing
    } else if state == target {
        *best = count;
    } else if index < buttons.len() {
        get_lowest_button_presses(target, buttons, state ^ buttons[index], index + 1, count + 1, best);
        get_lowest_button_presses(target, buttons, state, index + 1, count, best);
    }
}

fn part_1(input: &str) -> u32 {
    let mut buttons = Vec::new();

    input.lines().map(|line| {
        let part_cnt = line.split_whitespace().count();
        let mut parts = line.split_whitespace();
        let target = parts.next().unwrap().trim_matches(['[', ']']).chars().enumerate().fold(0, |acc, (n, c)|
            acc | if c == '#' { 1 << n } else { 0 }
        );
        buttons.extend(parts.take(part_cnt - 2).map(|p|
            p
                .trim_matches(['(', ')'])
                .split(',')
                .map(|num| num.parse::<u32>().unwrap())
                .fold(0, |acc, x| acc | (1 << x))
        ));

        let mut best = u32::MAX;
        get_lowest_button_presses(target, &buttons, 0, 0, 0, &mut best);
        buttons.clear();
        best
    }).sum()
}

fn part_2(input: &str) -> u32 {
    let mut buttons = Vec::new();
    let mut joltages = Vec::new();
    let mut rhs = Vec::new();
    let mut grid = Vec::new();
    let mut max_presses_per_trailing_button = Vec::new();
    let mut press_difference_per_press = Vec::new();
    let mut factors = Vec::new();
    let mut to_check = BinaryHeap::new();
    let mut checked_states = HashSet::new();
    let mut trailing_presses = Vec::new();
    let mut presses = Vec::new();

    input.lines().map(|line| {
        grid.clear();
        buttons.clear();
        joltages.clear();
        rhs.clear();
        max_presses_per_trailing_button.clear();
        press_difference_per_press.clear();
        factors.clear();
        to_check.clear();
        checked_states.clear();

        let part_cnt = line.split_whitespace().count();
        let mut parts = line.split_whitespace().skip(1);
        buttons.extend((0..part_cnt - 2).map(|_|
            parts
                .next()
                .unwrap()
                .trim_matches(['(', ')'])
                .split(',')
                .map(|num| num.parse::<u32>().unwrap())
                .fold(0u32, |acc, x| acc | (1 << x))
        ));
        joltages.extend(
            parts
                .next()
                .unwrap()
                .trim_matches(['{', '}'])
                .split(',')
                .map(|p| Rational32::from_integer(p.parse::<i32>().unwrap()))
        );

        rhs.extend(joltages.iter().copied());
        let rows = rhs.len();
        let columns = buttons.len();
        grid.extend((0..rows).flat_map(|row| {
            let buttons = &buttons;
            (0..columns).map(move |column|
                if buttons[column] & (1 << row) != 0 { Rational32::from_integer(1) } else { 0.into() }
            )
        }));

        for i in 0..rows.min(columns) {
            let pivot = (i..columns)
                .flat_map(|column| (i..rows).map(move |row| (row, column)))
                .find(|(row, column)| grid[row * columns + column] != 0.into());
            if pivot.is_none() {
                break;
            }
            let (row, column) = pivot.unwrap();
            if row != i {
                for c in 0..columns {
                    grid.swap(i * columns + c, row * columns + c);
                }
                rhs.swap(i, row);
            }

            if column != i {
                for r in 0..rows {
                    grid.swap(r * columns + i, r * columns + column);
                }
                buttons.swap(i, column);
            }

            if grid[i * columns + i] != 1.into() {
                let denom = grid[i * columns + i];
                for c in i..columns {
                    grid[i * columns + c] /= denom;
                }
                rhs[i] /= denom;
            }
            assert!(grid[i * columns + i] == 1.into());

            for r in 0..rows {
                if r != i && grid[r * columns + i] != 0.into() {
                    let factor = grid[r * columns + i] / grid[i * columns + i];
                    for c in i..columns {
                        let subtrahend = factor * grid[i * columns + c];
                        grid[r * columns + c] -= subtrahend;
                    }
                    let subtrahend = factor * rhs[i];
                    rhs[r] -= subtrahend;
                }
            }
        }

        let num_nonzero_rows = (0..rows).rev().find(|&r| {
            (0..columns).any(|c| grid[r * columns + c] != 0.into())
        }).unwrap() + 1;

        assert!(rhs.iter().skip(num_nonzero_rows).all(|&x| x == 0.into()));
        rhs.truncate(num_nonzero_rows);
        grid.truncate(num_nonzero_rows * columns);
        let rows = rhs.len();

        max_presses_per_trailing_button.extend((rows..columns).map(|c| {
            let buttons = &buttons;
            joltages.iter().enumerate().filter_map(move |(n, &joltage)|
                if buttons[c] & (1 << n) != 0 { Some(joltage.to_integer() as u32) } else { None }
            ).min().unwrap()
        }));

        press_difference_per_press.extend((rows..columns).map(|c| {
            Rational32::from_integer(1) - (0..rows).map(|r| grid[r * columns + c]).sum::<Rational32>()
        }));

        let mut factor = 1;
        factors.extend(max_presses_per_trailing_button.iter().map(|x| {
            let curr_factor = factor;
            factor *= x + 1;
            curr_factor
        }));

        let start = press_difference_per_press.iter().zip(&max_presses_per_trailing_button).map(|(&diff, &max)| {
            if diff < 0.into() {
                max
            } else {
                0
            }
        }).enumerate().fold(0, |acc, (i, presses)|
            acc + presses * factors[i]
        );

        to_check.push((Reverse(Rational32::from_integer(0)), start));

        while let Some((Reverse(added_presses), at)) = to_check.pop() {
            trailing_presses.clear();
            presses.clear();
            trailing_presses.extend(factors.iter().zip(&max_presses_per_trailing_button).map(|(f, max)|
                at / f % (max + 1)
            ));

            presses.extend((0..rows).map(|r| {
                rhs[r] - (0..(columns - rows)).map(|i| {
                    grid[r * columns + (rows + i)] * Rational32::from_integer(trailing_presses[i] as i32)
                }).sum::<Rational32>()
            }));

            if presses.iter().all(|&p| p >= 0.into() && p.is_integer()) {
                let tot_presses = presses
                    .iter()
                    .map(|&p| p.to_integer() as u32)
                    .sum::<u32>() + trailing_presses.iter().sum::<u32>();
                return tot_presses;
            }

            for (((&presses, &diff), &factor), &max) in trailing_presses
                    .iter()
                    .zip(&press_difference_per_press)
                    .zip(&factors)
                    .zip(&max_presses_per_trailing_button) {
                if let Some((new_presses, new_added_presses)) = if diff < 0.into() && presses > 0 {
                    Some((presses - 1, added_presses - diff))
                } else if presses < max {
                    Some((presses + 1, added_presses + diff))
                } else {
                    None
                } {
                    let new_at = at - presses * factor + new_presses * factor;
                    if checked_states.insert(new_at) {
                        to_check.push((Reverse(new_added_presses), new_at));
                    }
                }
            }
        }

        panic!("Found no solution");
    }).sum()
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let start_time = std::time::Instant::now();
    let result = part_1(&input);
    println!("Part 1 time: {:?}", std::time::Instant::now() - start_time);
    println!("Part 1 result: {}", result);

    let start_time = std::time::Instant::now();
    let result = part_2(&input);
    println!("Part 2 time: {:?}", std::time::Instant::now() - start_time);
    println!("Part 2 result: {}", result);
}
