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

    result.into()
}

pub fn part_two_bruteforce(input: &str) -> i128 {
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

    loop {
        for direction in directions.clone() {
            let mut all_output_ok = 0;
            let mut new_position = Vec::new();
            result += 1;
            for position in &positions {
                new_position.push(
                    map[position[0] as usize][position[1] as usize][position[2] as usize]
                        [direction as usize],
                );
                if position[2] == 25 {
                    all_output_ok += 1;
                }
            }
            positions = new_position;
            if all_output_ok == positions.len() {
                return result.into();
            }
        }
    }

    result.into()
}

pub fn part_two(input: &str) -> i128 {
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

    let mut cycles: Vec<(u32, u32)> = Default::default();
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
                        first_step + first_iteration * directions.len() as u32,
                        direction_list_iterations * directions.len() as u32 + step as u32
                            - first_step
                            + first_iteration * directions.len() as u32,
                    ));
                    break 'compute_cycle;
                }

                if position[2] == 25 {
                    zs.push(step as u32);
                }

                walked_map[position[0] as usize][position[1] as usize][position[2] as usize]
                    .push((step as u32, direction_list_iterations));
            }
            direction_list_iterations += 1;
        }

        z_positions.push(zs.clone());
    }

    // println!("{:?}", cycles);
    // println!("{:?}", z_positions);

    result.into()
}
