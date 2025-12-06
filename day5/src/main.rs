use std::io::Read;

fn part_1(input: &str) -> usize {
    let mut lines = input.lines();

    let mut ranges = Vec::<[u64; 2]>::new();
    while let Some(line) = lines.next() {
        if line.trim().is_empty() {
            break;
        }

        let mut parts = line.split("-");
        let start = parts.next().unwrap().parse().unwrap();
        let end = parts.next().unwrap().parse().unwrap();
        ranges.push([start, end]);
    }

    ranges.sort_by_key(|r| r[0]);

    let mut ranges_merged = Vec::<[u64; 2]>::new();
    for range in ranges {
        if let Some(last) = ranges_merged.last_mut() {
            if range[0] <= last[1] + 1 {
                last[1] = last[1].max(range[1]);
                continue;
            }
        }
        ranges_merged.push(range);
    }

    lines.filter(|line| {
        let ingredient: u64 = line.parse().unwrap();
        ranges_merged.binary_search_by(|&[start, end]| if ingredient < start {
            std::cmp::Ordering::Greater
        } else if ingredient > end {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Equal
        }).is_ok()
    }).count()
}

fn part_2(input: &str) -> u64 {
    let mut lines = input.lines();

    let mut ranges = Vec::<[u64; 2]>::new();
    while let Some(line) = lines.next() {
        if line.trim().is_empty() {
            break;
        }

        let mut parts = line.split("-");
        let start = parts.next().unwrap().parse().unwrap();
        let end = parts.next().unwrap().parse().unwrap();
        ranges.push([start, end]);
    }

    ranges.sort_by_key(|r| r[0]);

    let mut ranges_merged = Vec::<[u64; 2]>::new();
    for range in ranges {
        if let Some(last) = ranges_merged.last_mut() {
            if range[0] <= last[1] + 1 {
                last[1] = last[1].max(range[1]);
                continue;
            }
        }
        ranges_merged.push(range);
    }

    ranges_merged.into_iter().map(|[start, end]| end - start + 1).sum()
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
