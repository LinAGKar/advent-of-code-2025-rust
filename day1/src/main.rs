use std::io::Read;

const MOD: i16 = 100;

fn part_1(input: &str) -> u16 {
    input.lines().fold((50, 0), |(pos, count), line| {
        let direction = line.chars().next().unwrap();
        let distance = line[1..].parse::<i16>().unwrap();
        let new_pos = match direction {
            'L' => pos - distance,
            'R' => pos + distance,
            _ => panic!("Invalid direction"),
        }.rem_euclid(MOD);
        (new_pos, if new_pos == 0 { count + 1 } else { count })
    }).1
}

fn part_2(input: &str) -> u32 {
    input.lines().fold((50, 0), |(pos, count), line| {
        let direction = line.chars().next().unwrap();
        let distance = line[1..].parse::<i16>().unwrap();
        let new_pos = match direction {
            'L' => pos - distance,
            'R' => pos + distance,
            _ => panic!("Invalid direction"),
        }.rem_euclid(MOD);
        let (start, end) = match direction {
            'L' => ((new_pos - 1).rem_euclid(MOD), (pos - 1).rem_euclid(MOD)),
            'R' => (pos, new_pos),
            _ => panic!("Invalid direction"),
        };

        (new_pos, count + if end < start { 1 } else { 0 } + distance.div_euclid(MOD) as u32)
    }).1
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
