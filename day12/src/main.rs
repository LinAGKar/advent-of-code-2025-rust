use std::{io::Read};

fn part_1(input: &str) -> usize {
    let mut parts = input.split("\n\n");

    let areas = (0..6)
        .map(|_| {
            parts
                .next()
                .unwrap()
                .lines()
                .skip(1)
                .flat_map(|line|
                    line.chars()
                )
                .filter(|&c| c == '#')
                .count() as u32
        })
        .collect::<Vec<_>>();

    parts.next().unwrap().lines().filter(|&line| {
        let mut parts = line.split(": ");
        let mut dimensions = [0; 2];
        for (dim, slot) in parts.next().unwrap().split('x').zip(&mut dimensions) {
            *slot = dim.parse::<u32>().unwrap();
        }
        let mut presents = [0; 6];
        for (count, slot) in parts.next().unwrap().split(' ').zip(&mut presents) {
            *slot = count.parse::<u32>().unwrap();
        }
        let area = dimensions.iter().product::<u32>();
        let three_by_threes = dimensions.iter().map(|&d| d / 3).product::<u32>();
        if three_by_threes >= presents.iter().sum() {
            return true;
        }
        let tot_area = presents.iter().enumerate().map(|(i, &p)| p * areas[i]).sum::<u32>();
        if tot_area > area {
            return false;
        }

        panic!("Inconclusive")
    }).count()
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let start_time = std::time::Instant::now();
    let result = part_1(&input);
    println!("Part 1 time: {:?}", std::time::Instant::now() - start_time);
    println!("Part 1 result: {}", result);
}
