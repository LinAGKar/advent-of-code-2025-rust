use std::io::Read;

fn part_1(input: &str) -> u32 {
    input.lines().map(|line| {
        let (pos, num) = line.char_indices().take(line.len() - 1).map(|(n, c)|
            (n, c.to_digit(10).unwrap())
        ).reduce(|(pos, num), (new_pos, new_num)|
            if new_num > num {
                (new_pos, new_num)
            } else {
                (pos, num)
            }
        ).unwrap();

        let num_2 = line[pos + 1..].chars().map(|c| c.to_digit(10).unwrap()).max().unwrap();

        num * 10 + num_2
    }).sum()
}

const DIGITS: usize = 12;

fn part_2(input: &str) -> u64 {
    input.lines().map(|line| {
        (0..DIGITS).fold((0, 0), |(pos, joltage), x| {
            let (new_pos, num) = line[pos..line.len() + x + 1 - DIGITS].char_indices().map(|(n, c)|
                (n + pos, c.to_digit(10).unwrap() as u64)
            ).reduce(|(pos, num), (new_pos, new_num)|
                if new_num > num {
                    (new_pos, new_num)
                } else {
                    (pos, num)
                }
            ).unwrap();

            (new_pos + 1, joltage * 10 + num)
        }).1
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
