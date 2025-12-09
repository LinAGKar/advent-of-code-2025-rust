use std::io::Read;

fn add_tiles_to_check<'a, T1: Iterator<Item = &'a [u32; 2]>, T2: Fn(u32, u32) -> bool>(
    tiles_to_check: &mut Vec<[u32; 2]>,
    tiles: T1,
    cmp: T2
) {
    for &tile in tiles {
        if tiles_to_check.is_empty() {
            tiles_to_check.push(tile);
        } else if cmp(tile[1], tiles_to_check.last().unwrap()[1]) {
            if tile[0] == tiles_to_check.last().unwrap()[0] {
                tiles_to_check.pop();
            }
            tiles_to_check.push(tile);
        }
    }
}

fn part_1(input: &str) -> u64 {
    let mut tiles = input
        .lines()
        .map(|line| {
            let mut tile = [0u32; 2];
            for (src, dst) in line.split(',').zip(&mut tile) {
                *dst = src.parse().unwrap();
            }
            tile
        })
        .collect::<Vec<_>>();

    tiles.sort_unstable_by_key(|x| x[0]);

    let mut tiles_to_check = [const { Vec::new() }; 4];
    add_tiles_to_check(&mut tiles_to_check[0], tiles.iter(), |a, b| a > b);
    add_tiles_to_check(&mut tiles_to_check[1], tiles.iter(), |a, b| a < b);
    add_tiles_to_check(&mut tiles_to_check[2], tiles.iter().rev(), |a, b| a > b);
    add_tiles_to_check(&mut tiles_to_check[3], tiles.iter().rev(), |a, b| a < b);

    [
        (&tiles_to_check[0], &tiles_to_check[3], true),
        (&tiles_to_check[1], &tiles_to_check[2], false),
    ].into_iter().flat_map(|(a, b, swap_y)|
        a.iter().flat_map(|a| b.iter().map(move |b| (a, b))).filter_map(move |(&[ax, mut ay], &[bx, mut by])| {
            if swap_y {
                (by, ay) = (ay, by);
            }

            if bx >= ax && by >= ay {
                Some((bx - ax + 1) as u64 * (by - ay + 1) as u64)
            } else {
                None
            }
        })
    ).max().unwrap()
}

fn part_2(input: &str) -> u64 {
    let tiles = input
        .lines()
        .map(|line| {
            let mut tile = [0u32; 2];
            for (src, dst) in line.split(',').zip(&mut tile) {
                *dst = src.parse().unwrap();
            }
            tile
        })
        .collect::<Vec<_>>();

    let edges = tiles.iter().zip(tiles.iter().skip(1).chain(&[tiles[0]])).map(|(&[ax, ay], &[bx, by])| {
        [ax.min(bx), ay.min(by), ax.max(bx), ay.max(by)]
    }).collect::<Vec<_>>();

    tiles.iter().enumerate().flat_map(|(n, &a)| {
        let edges = &edges;

        tiles.iter().skip(n + 1).filter_map(move |b| {
            let corner_ax = a[0].min(b[0]);
            let corner_bx = a[0].max(b[0]);
            let corner_ay = a[1].min(b[1]);
            let corner_by = a[1].max(b[1]);

            if edges.iter().any(move |&[ex1, ey1, ex2, ey2]|
                ex2 > corner_ax && ex1 < corner_bx &&
                ey2 > corner_ay && ey1 < corner_by
            ) {
                None
            } else {
                Some((corner_bx - corner_ax + 1) as u64 * (corner_by - corner_ay + 1) as u64)
            }
        })
    }).max().unwrap()
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
