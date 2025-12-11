use std::{collections::HashMap, io::Read};

fn find_number_of_paths(neighbors: &Vec<Vec<u16>>, from: u16, to: u16, memo: &mut HashMap<u16, u32>) -> u32 {
    if let Some(&count) = memo.get(&from) {
        count
    } else if from == to {
        1
    } else {
        let count = neighbors[from as usize].iter().map(|&next| find_number_of_paths(neighbors, next, to, memo)).sum();
        memo.insert(from, count);
        count
    }
}

fn part_1(input: &str) -> u32 {
    let mut index_by_name = HashMap::new();

    let mut get_index = |name| {
        let next_index = index_by_name.len() as u16;
        *index_by_name.entry(name).or_insert(next_index)
    };

    let you_index = get_index("you");
    let out_index = get_index("out");

    let mut neighbors = Vec::new();

    for line in input.lines() {
        let mut parts = line.split(": ");
        let from = get_index(parts.next().unwrap());
        if neighbors.len() <= from as usize {
            neighbors.resize(from as usize + 1, Vec::new());
        }
        neighbors[from as usize].extend(parts.next().unwrap().split(' ').map(|to_name| {
            get_index(to_name)
        }));
    }

    find_number_of_paths(&neighbors, you_index, out_index, &mut HashMap::new())
}

fn find_number_of_paths_w_intermediates(
    neighbors: &Vec<Vec<u16>>,
    from: u16,
    to: u16,
    nodes_passed: u8,
    nodes_to_pass: &[u16; 2],
    memo: &mut HashMap<(u16, u8), u64>,
) -> u64 {
    if let Some(&count) = memo.get(&(from, nodes_passed)) {
        count
    } else if from == to && nodes_passed == 0b11 {
        1
    } else {
        let nodes_passed = nodes_passed |
            nodes_to_pass
            .iter()
            .enumerate()
            .fold(0, |acc, (i, &node)| if from == node { acc | (1 << i) } else { acc });
        let count = neighbors[from as usize]
            .iter()
            .map(|&next| find_number_of_paths_w_intermediates(neighbors, next, to, nodes_passed, nodes_to_pass, memo))
            .sum();
        memo.insert((from, nodes_passed), count);
        count
    }
}

fn part_2(input: &str) -> u64 {
    let mut index_by_name = HashMap::new();

    let mut get_index = |name| {
        let next_index = index_by_name.len() as u16;
        *index_by_name.entry(name).or_insert(next_index)
    };

    let svr_index = get_index("svr");
    let out_index = get_index("out");
    let dac_index = get_index("dac");
    let fft_index = get_index("fft");

    let mut neighbors = Vec::new();

    for line in input.lines() {
        let mut parts = line.split(": ");
        let from = get_index(parts.next().unwrap());
        if neighbors.len() <= from as usize {
            neighbors.resize(from as usize + 1, Vec::new());
        }
        neighbors[from as usize].extend(parts.next().unwrap().split(' ').map(|to_name| {
            get_index(to_name)
        }));
    }

    find_number_of_paths_w_intermediates(
        &neighbors, svr_index, out_index, 0, &[dac_index, fft_index], &mut HashMap::new(),
    )
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
