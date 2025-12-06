use std::io::Read;

fn part_1(input: &str) -> u64 {
    let mut parts: Vec<_> = input.lines().map(|line| line.split_ascii_whitespace()).collect();
    parts.pop().unwrap().map(|operator|
        parts.iter_mut().map(|line| line.next().unwrap().parse::<u64>().unwrap()).reduce(|a, b| match operator {
            "+" => a + b,
            "*" => a * b,
            x => panic!("Unknown operator {x}"),
        }).unwrap()
    ).sum()
}

fn part_2(input: &str) -> u64 {
    let lines: Vec<_> = input.lines().collect();

    let mut numbers: Vec<_> = lines[..lines.len() - 1].iter().map(|line| line.chars()).collect();
    let mut operators = lines.last().unwrap().chars();

    let mut tot: u64 = 0;
    let mut curr: u64 = 0;
    let mut curr_op = ' ';
    while let Some(operator) = operators.next() {
        match operator {
            '+' => {
                tot += curr;
                curr = 0;
                curr_op = operator;
            }

            '*' => {
                tot += curr;
                curr = 1;
                curr_op = operator;
            }

            ' ' => {}

            x => panic!("Unknown operator {x}"),
        }

        let operand = numbers
            .iter_mut()
            .filter_map(|number| number.next())
            .filter_map(|x| x.to_digit(10))
            .map(|d| d as u64)
            .reduce(|acc, x| acc * 10 + x);

        match (curr_op, operand) {
            ('+', Some(operand)) => curr += operand,
            ('*', Some(operand)) => curr *= operand,
            _ => {}
        }
    }

    tot += curr;

    tot
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
