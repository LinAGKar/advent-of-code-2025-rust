use std::io::Read;

fn part_1(input: &str) -> u32 {
    let junction_boxes = input
        .lines()
        .map(|line| {
            let mut coord = [0; 3];
            for (c, num) in coord.iter_mut().zip(line.split(',')) {
                *c = num.parse::<u32>().unwrap();
            }
            coord
        })
        .collect::<Vec<_>>();

    let mut distances = junction_boxes.iter().enumerate().flat_map(|(n, box1)| {
        junction_boxes.iter().enumerate().skip(n + 1).map(move |(m, box2)|
            (
                box1.iter()
                    .zip(box2.iter())
                    .map(|(a, b)| (if a > b { a - b } else { b - a } as u64).pow(2))
                    .sum::<u64>(),
                n as u16,
                m as u16,
            )
        )
    }).collect::<Vec<_>>();

    distances.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut neighbors_by_box = vec![Vec::<u16>::new(); junction_boxes.len()];

    for &i in distances.iter().take(1000) {
        neighbors_by_box[i.1 as usize].push(i.2);
        neighbors_by_box[i.2 as usize].push(i.1);
    }

    let mut box_in_circuit = vec![false; junction_boxes.len()];
    let mut to_check = Vec::new();
    let mut top_sizes = [0; 3];

    for i in 0..junction_boxes.len() {
        let mut circuit_size: u16 = 0;
        to_check.push(i);

        while let Some(box_index) = to_check.pop() {
            if box_in_circuit[box_index] {
                continue;
            }

            box_in_circuit[box_index] = true;
            circuit_size += 1;
            to_check.extend(neighbors_by_box[box_index].iter().map(|&index| index as usize));
        }

        if circuit_size > top_sizes[0] {
            top_sizes[0] = circuit_size;
            top_sizes.sort_unstable();
        }
    }

    top_sizes.iter().map(|&s| s as u32).product()
}

fn part_2(input: &str) -> u64 {
    let junction_boxes = input
        .lines()
        .map(|line| {
            let mut coord = [0; 3];
            for (c, num) in coord.iter_mut().zip(line.split(',')) {
                *c = num.parse::<u32>().unwrap();
            }
            coord
        })
        .collect::<Vec<_>>();

    let mut distances = junction_boxes.iter().enumerate().flat_map(|(n, box1)| {
        junction_boxes.iter().enumerate().skip(n + 1).map(move |(m, box2)|
            (
                box1.iter()
                    .zip(box2.iter())
                    .map(|(a, b)| (if a > b { a - b } else { b - a } as u64).pow(2))
                    .sum::<u64>(),
                n as u16,
                m as u16,
            )
        )
    }).collect::<Vec<_>>();

    distances.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut neighbors_by_box = vec![Vec::<usize>::new(); junction_boxes.len()];
    let mut circuit_by_box = (0..junction_boxes.len()).map(|i| i as u16).collect::<Vec<_>>();
    let mut to_change = Vec::new();
    let mut circuit_count = junction_boxes.len();

    for (_, a, b) in distances {
        let a = a as usize;
        let b = b as usize;
        let circuit = circuit_by_box[a];

        if circuit_by_box[b] != circuit {
            to_change.push(b);

            while let Some(box_index) = to_change.pop() {
                if circuit_by_box[box_index] == circuit {
                    continue;
                }

                circuit_by_box[box_index] = circuit;
                to_change.extend(neighbors_by_box[box_index].iter().cloned());
            }

            // Don't bother adding another connection between boxes already in the same circuit
            neighbors_by_box[a].push(b);
            neighbors_by_box[b].push(a);
            circuit_count -= 1;

            if circuit_count == 1 {
                return junction_boxes[a][0] as u64 * junction_boxes[b][0] as u64;
            }
        }
    }

    unreachable!()
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
