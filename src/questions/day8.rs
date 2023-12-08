pub fn part_one(input: &str) -> u64 {
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
                return result;
            }
        }
    }

    result
}

pub fn part_two(input: &str) -> u64 {
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

    println!("{:?}", positions);

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
                return result;
            }
        }
    }

    result
}

pub fn part_two_not_bruteforce(input: &str) -> u64 {
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

    for starting_position in &positions {
        let mut walked_map: [[[Vec<(u8, u32)>; 26]; 26]; 26] = Default::default();
        let mut position = starting_position;

        let mut zs = Vec::new();
        let mut cycle = false;
        let mut direction_list_iterations: u32 = 0;

        println!(
            "\n------------- {} -------------",
            position
                .iter()
                .map(|&c| (c + b'A') as char)
                .collect::<String>()
        );

        'compute_cycle: loop {
            for (step, direction) in directions.iter().enumerate() {
                position = &map[position[0] as usize][position[1] as usize][position[2] as usize]
                    [*direction as usize];

                walked_map[position[0] as usize][position[1] as usize][position[2] as usize]
                    .push((step as u8, direction_list_iterations));

                if let Some((first_step, first_iteration)) = walked_map[position[0] as usize]
                    [position[1] as usize][position[2] as usize]
                    .iter()
                    .find(|&&(left, right)| {
                        left == step as u8 && right != direction_list_iterations
                    })
                { 
                    if !cycle {
                        /* println!(
                            "found cycle at step {}",
                            step + direction_list_iterations as usize * directions.len()
                        );
                        println!(
                            "it spans over {} iterations, starting at iteration {} step {}",
                            direction_list_iterations - first_iteration,
                            first_iteration,
                            first_step
                        );
                        println!("Zs: {:?}", zs);
                        println!(
                            "Position: {}",
                            position
                                .iter()
                                .map(|&c| (c + b'A') as char)
                                .collect::<String>()
                        ); */
                        cycle = true;
                    }
                    // println!("WalkTile: {:?}", walked_map[position[0] as usize][position[1] as usize][position[2] as usize]);
                    // break 'compute_cycle;
                }

                if position[2] == 25 {
                    zs.push(position);
                }
            }

            direction_list_iterations += 1;
        }
    }

    result
}
