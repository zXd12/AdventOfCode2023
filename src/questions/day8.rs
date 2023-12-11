pub fn part_one(input: &str) -> i128 {
    let mut result: u64 = 0;
    let mut lines = input.lines();

    let directions: Vec<bool> = lines
        .next()
        .unwrap()
        .as_bytes()
        .iter()
        .map(|a| *a == b'R')
        .collect();
    lines.next();

    let mut map: [[[[[u8; 3]; 2]; 26]; 26]; 26] = [[[[[26; 3]; 2]; 26]; 26]; 26];

    for line in lines {
        let bytes = line.as_bytes();
        map[(bytes[0] - b'A') as usize][(bytes[1] - b'A') as usize][(bytes[2] - b'A') as usize] = [
            [bytes[7] - b'A', bytes[8] - b'A', bytes[9] - b'A'],
            [bytes[12] - b'A', bytes[13] - b'A', bytes[14] - b'A'],
        ];
    }

    let mut position: [u8; 3] = [0, 0, 0];

    loop {
        for direction in directions.clone() {
            result += 1;
            position = map[position[0] as usize][position[1] as usize][position[2] as usize]
                [direction as usize];
            if position == [25, 25, 25] {
                return result.into();
            }
        }
    }
}

pub fn part_two(input: &str) -> i128 {
    let mut lines = input.lines();

    let directions: Vec<bool> = lines
        .next()
        .unwrap()
        .as_bytes()
        .iter()
        .map(|a| *a == b'R')
        .collect();
    lines.next();

    let mut map: [[[[[u8; 3]; 2]; 26]; 26]; 26] = [[[[[26; 3]; 2]; 26]; 26]; 26];
    let mut positions: Vec<[u8; 3]> = Vec::new();

    for line in lines {
        let bytes = line.as_bytes();
        map[(bytes[0] - b'A') as usize][(bytes[1] - b'A') as usize][(bytes[2] - b'A') as usize] = [
            [bytes[7] - b'A', bytes[8] - b'A', bytes[9] - b'A'],
            [bytes[12] - b'A', bytes[13] - b'A', bytes[14] - b'A'],
        ];
        if bytes[2] == b'A' {
            positions.push([bytes[0] - b'A', bytes[1] - b'A', bytes[2] - b'A']);
        }
    }

    let mut cycles: Vec<(u64, u64)> = Default::default();
    let mut z_positions: Vec<Vec<u32>> = Default::default();

    for starting_position in &positions {
        let mut walked_map: [[[Vec<(u32, u32)>; 26]; 26]; 26] = Default::default();
        let mut position = starting_position;

        let mut zs = Vec::new();
        let mut direction_list_iterations: u32 = 0;

        walked_map[position[0] as usize][position[1] as usize][position[2] as usize].push((0, 0));

        'compute_cycle: loop {
            for (step, direction) in directions.clone().iter().enumerate() {
                position = &map[position[0] as usize][position[1] as usize][position[2] as usize]
                    [*direction as usize];

                if let Some((first_step, first_iteration)) = walked_map[position[0] as usize]
                    [position[1] as usize][position[2] as usize]
                    .iter()
                    .find(|&&(left, _)| left == step as u32)
                {
                    cycles.push((
                        (first_step + first_iteration) as u64 * directions.len() as u64,
                        (direction_list_iterations * directions.len() as u32 + step as u32
                            - first_step
                            + first_iteration * directions.len() as u32) as u64,
                    ));
                    break 'compute_cycle;
                }

                if position[2] == 25 {
                    zs.push(step as u32 + directions.len() as u32 * direction_list_iterations);
                }

                walked_map[position[0] as usize][position[1] as usize][position[2] as usize]
                    .push((step as u32, direction_list_iterations));
            }
            direction_list_iterations += 1;
        }

        z_positions.push(zs.clone());
    }

    cycles[0..].iter().skip(1).fold(cycles[0], |acc, cycle| cr(acc, *cycle)).1.into()
    // cycles[0..].iter().skip(1).fold(cycles[0].1, |acc,(_, cycle_size) | lcm(acc, *cycle_size)).into()
}

fn gcd(number1: u64, number2: u64) -> u64 {
    let mut biggest_number = number1.max(number2);
    let mut smallest_number = number1.min(number2);
    let mut remainder = biggest_number%smallest_number;
    while remainder != 0 {
        biggest_number = smallest_number;
        smallest_number = remainder;
        remainder = biggest_number%smallest_number;
    }
    smallest_number
}

fn lcm(number1: u64, number2: u64) -> u64 {
    number1 * number2 / gcd(number1, number2)
}

fn cr(cycle1: (u64, u64), cycle2: (u64, u64)) -> (u64, u64) {
    let mut cycle1_value = cycle1.0;
    while cycle1_value%cycle2.1 != cycle2.0 {
        cycle1_value += cycle1.1%cycle2.1;
    }
    (cycle1_value, lcm(cycle1.1, cycle2.1))
}