use std::{collections::VecDeque, io::Read};

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Splitter,
    Beam,
}

fn part_1(input: &str) -> u32 {
    let width = input.lines().next().unwrap().len();
    let mut beams = Vec::new();

    let mut grid: Vec<_> = input.chars().filter(|&c| c != '\n').enumerate().map(|(n, c)| match c {
        '.' => Tile::Empty,
        '^' => Tile::Splitter,
        'S' => {
            beams.push(n);
            Tile::Beam
        }
        _ => panic!("Unexpected character at position {}", n),
    }).collect();

    let mut count = 0;

    while let Some(idx) = beams.pop() {
        let new_idx = idx + width;
        if new_idx >= grid.len() {
            continue;
        }

        match grid[new_idx] {
            Tile::Empty => {
                grid[new_idx] = Tile::Beam;
                beams.push(new_idx);
            }
            Tile::Splitter => {
                count += 1;
                 for new_idx in [new_idx - 1, new_idx + 1] {
                    if grid[new_idx] == Tile::Empty {
                        grid[new_idx] = Tile::Beam;
                        beams.push(new_idx);
                    }
                 }
            }
            Tile::Beam => {}
        }
    }

    count
}

#[derive(Clone, Copy, PartialEq)]
enum QuantumTile {
    Empty,
    Splitter,
    Beam(u64),
}

fn part_2(input: &str) -> u64 {
    let width = input.lines().next().unwrap().len();
    let mut beams = VecDeque::new();

    let mut grid: Vec<_> = input.chars().filter(|&c| c != '\n').enumerate().map(|(n, c)| match c {
        '.' => QuantumTile::Empty,
        '^' => QuantumTile::Splitter,
        'S' => {
            beams.push_back(n);
            QuantumTile::Beam(1)
        }
        _ => panic!("Unexpected character at position {}", n),
    }).collect();

    let mut tot_count = 0;

    while let Some(idx) = beams.pop_front() {
        let count = if let QuantumTile::Beam(count) = grid[idx] {
            count
        } else {
            panic!("Expected beam at position {}", idx);
        };

        let new_idx = idx + width;
        if new_idx >= grid.len() {
            tot_count += count;
            continue;
        }

        match grid[new_idx] {
            QuantumTile::Empty => {
                grid[new_idx] = QuantumTile::Beam(count);
                beams.push_back(new_idx);
            }
            QuantumTile::Splitter => {
                 for new_idx in [new_idx - 1, new_idx + 1] {
                    match grid[new_idx] {
                        QuantumTile::Beam(ref mut c) => {
                            *c += count;
                        }
                        QuantumTile::Empty => {
                            grid[new_idx] = QuantumTile::Beam(count);
                            beams.push_back(new_idx);
                        }
                        QuantumTile::Splitter => panic!("Unexpected splitter at position {}", new_idx),
                    }
                 }
            }
            QuantumTile::Beam(ref mut c) => {
                *c += count;
            }
        }
    }

    tot_count
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
