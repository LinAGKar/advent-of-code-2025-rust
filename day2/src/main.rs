use std::io::Read;

fn part_1(input: &str) -> u64 {
    input.trim().split(',').map(|range| {
        let mut parts = range.split('-');
        let start = parts.next().unwrap();
        let start_count = start.chars().count();
        let start = start.parse().unwrap();
        let end = parts.next().unwrap();
        let end_count = end.chars().count();
        let end = end.parse().unwrap();
        (start_count..=end_count).filter(|count| count % 2 == 0).map(|count| {
            let start = if count == start_count { start } else { 10u64.pow(count as u32 - 1) };
            let end = if count == end_count { end } else { 10u64.pow(count as u32) - 1 };
            let denom = 10u64.pow((count / 2) as u32);
            let start_part = start / denom;
            let end_part = end / denom;
            let start_part = if start <= start_part * denom + start_part { start_part } else { start_part + 1 };
            let end_part = if end >= end_part * denom + end_part { end_part } else { end_part - 1 };
            (start_part..=end_part).map(|x| x * denom + x).sum::<u64>()
        }).sum::<u64>()
    }).sum::<u64>()
}

fn build_num(part: u64, repeat_count: usize, denom: u64) -> u64 {
    (0..repeat_count).fold(0, |acc, _| acc * denom + part)
}

fn part_2(input: &str) -> u64 {
    let mut dedup = std::collections::HashSet::new();

    input.trim().split(',').map(|range| {
        let mut parts = range.split('-');
        let start = parts.next().unwrap();
        let start_count = start.chars().count();
        let start = start.parse().unwrap();
        let end = parts.next().unwrap();
        let end_count = end.chars().count();
        let end = end.parse().unwrap();
        (start_count..=end_count).map(|count| {
            dedup.clear();
            (1..count).filter(|&part_count| (count % part_count) == 0).map(|part_count| {
                let start = if count == start_count { start } else { 10u64.pow(count as u32 - 1) };
                let end = if count == end_count { end } else { 10u64.pow(count as u32) - 1 };
                let denom = 10u64.pow((part_count) as u32);
                let repeat_count = count / part_count;
                let denom2 = denom.pow((repeat_count - 1) as u32);
                let start_part = start / denom2;
                let end_part = end / denom2;
                let start_part = if start <= build_num(start_part, repeat_count, denom) { start_part } else { start_part + 1 };
                let end_part = if end >= build_num(end_part, repeat_count, denom) { end_part } else { end_part - 1 };
                (start_part..=end_part).map(|x| build_num(x, repeat_count, denom)).filter(|&x| {
                    let include = !dedup.contains(&x);
                    if include {
                        dedup.insert(x);
                    }
                    include
                }).sum::<u64>()
            }).sum::<u64>()
        }).sum::<u64>()
    }).sum::<u64>()
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
