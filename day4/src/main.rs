use std::io::Read;

fn part_1(input: &str) -> usize {
    let grid: Vec<_> = input.chars().filter_map(|c| match c {
        '\n' => None,
        other => Some(other == '@'),
    }).collect();

    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    (0..height).map(|y|
        (0..width).filter(|&x|
            grid[y * width + x] && [
                [x.wrapping_sub(1), y.wrapping_sub(1)],
                [x.wrapping_sub(1), y],
                [x.wrapping_sub(1), y + 1],
                [x, y.wrapping_sub(1)],
                [x, y + 1],
                [x + 1, y.wrapping_sub(1)],
                [x + 1, y],
                [x + 1, y + 1],
            ].into_iter().filter(|&[other_x, other_y]| other_x < width && other_y < height && grid[other_y * width + other_x]).count() < 4
        ).count()
    ).sum()
}

fn part_2(input: &str) -> u16 {
    let mut grid: Vec<_> = input.chars().filter_map(|c| match c {
        '\n' => None,
        other => Some(other == '@'),
    }).collect();

    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut to_remove = Vec::new();
    let mut meighbor_count: Vec<_ > = (0..height).flat_map(|y| (0..width).map(move |x| (x, y))).map(|(x, y)| {
        let count = [
            [x.wrapping_sub(1), y.wrapping_sub(1)],
            [x.wrapping_sub(1), y],
            [x.wrapping_sub(1), y + 1],
            [x, y.wrapping_sub(1)],
            [x, y + 1],
            [x + 1, y.wrapping_sub(1)],
            [x + 1, y],
            [x + 1, y + 1],
        ].into_iter().filter(|&[other_x, other_y]| other_x < width && other_y < height && grid[other_y * width + other_x]).count();
        if count < 4 && grid[y * width + x] {
            to_remove.push((x, y));
        }
        count
    }).collect();

    let mut count = 0;

    while let Some((x, y)) = to_remove.pop() {
        count += 1;
        grid[y * width + x] = false;

        for &[other_x, other_y] in &[
            [x.wrapping_sub(1), y.wrapping_sub(1)],
            [x.wrapping_sub(1), y],
            [x.wrapping_sub(1), y + 1],
            [x, y.wrapping_sub(1)],
            [x, y + 1],
            [x + 1, y.wrapping_sub(1)],
            [x + 1, y],
            [x + 1, y + 1],
        ] {
            if other_x < width && other_y < height && grid[other_y * width + other_x] {
                meighbor_count[other_y * width + other_x] -= 1;
                if meighbor_count[other_y * width + other_x] == 3 {
                    to_remove.push((other_x, other_y));
                }
            }
        }
    }

    count
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
