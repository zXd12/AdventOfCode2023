pub fn part_one(input: &str) -> i128 {
    let mut result = u64::MAX;

    let mut lines = input.lines();
    let bytes = lines.next().unwrap().as_bytes();

    let seeds: Vec<u64> = bytes[7..]
        .split(|&c| c == b' ')
        .map(|chunk| {
            let num = chunk
                .iter()
                .map(|&c| c - b'0')
                .fold(0, |acc: u64, f| acc * 10 + f as u64);
            num
        })
        .collect();

    lines.next();

    let mut maps: [Vec<(u64, u64, u64)>; 7] = Default::default();
    let mut map_index = 0;

    'map_construction: loop {
        lines.next();

        loop {
            let line = lines.next();
            match line {
                Some(line) => {
                    if line.is_empty() {
                        map_index += 1;
                        break;
                    }
                    let bytes = line.as_bytes();
                    let mut mapping = (0, 0, 0);
                    let mut char_index = 0;

                    loop {
                        if bytes[char_index] == b' ' {
                            char_index += 1;
                            break;
                        }
                        if bytes[char_index] >= b'0' && bytes[char_index] <= b'9' {
                            mapping.1 *= 10;
                            mapping.1 += (bytes[char_index] - b'0') as u64;
                        }
                        char_index += 1;
                    }

                    loop {
                        if bytes[char_index] == b' ' {
                            char_index += 1;
                            break;
                        }
                        if bytes[char_index] >= b'0' && bytes[char_index] <= b'9' {
                            mapping.0 *= 10;
                            mapping.0 += (bytes[char_index] - b'0') as u64;
                        }
                        char_index += 1;
                    }

                    for i in char_index..bytes.len() {
                        if bytes[i] >= b'0' && bytes[i] <= b'9' {
                            mapping.2 *= 10;
                            mapping.2 += (bytes[i] - b'0') as u64;
                        }
                    }

                    maps[map_index].push(mapping);
                }
                None => break 'map_construction,
            }
        }
    }

    for mut seed in seeds {
        for map in &maps {
            for map in map.to_vec() {
                if seed >= map.0 && seed < map.0 + map.2 - 1 {
                    seed = map.1 + (seed - map.0);
                    break;
                }
            }
        }
        result = result.min(seed.try_into().unwrap());
    }

    result.into()
}

pub fn part_two(input: &str) -> i128 {
    let mut result = u64::MAX;

    let mut lines = input.lines();
    let bytes = lines.next().unwrap().as_bytes();

    let seed_ranges: Vec<(u64, u64)> = bytes[7..]
        .split(|&c| c == b' ')
        .map(|chunk| {
            chunk
                .iter()
                .map(|&c| c - b'0')
                .fold(0, |acc: u64, f| acc * 10 + f as u64)
        })
        .collect::<Vec<u64>>()
        .chunks_exact(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect();

    lines.next();

    let mut maps: [Vec<(u64, u64, u64)>; 7] = Default::default();
    let mut map_index = 0;

    'map_construction: loop {
        lines.next();

        loop {
            let line = lines.next();
            match line {
                Some(line) => {
                    if line.is_empty() {
                        map_index += 1;
                        break;
                    }
                    let bytes = line.as_bytes();
                    let mut mapping = (0, 0, 0);
                    let mut char_index = 0;

                    loop {
                        if bytes[char_index] == b' ' {
                            char_index += 1;
                            break;
                        }
                        if bytes[char_index] >= b'0' && bytes[char_index] <= b'9' {
                            mapping.1 *= 10;
                            mapping.1 += (bytes[char_index] - b'0') as u64;
                        }
                        char_index += 1;
                    }

                    loop {
                        if bytes[char_index] == b' ' {
                            char_index += 1;
                            break;
                        }
                        if bytes[char_index] >= b'0' && bytes[char_index] <= b'9' {
                            mapping.0 *= 10;
                            mapping.0 += (bytes[char_index] - b'0') as u64;
                        }
                        char_index += 1;
                    }

                    for i in char_index..bytes.len() {
                        if bytes[i] >= b'0' && bytes[i] <= b'9' {
                            mapping.2 *= 10;
                            mapping.2 += (bytes[i] - b'0') as u64;
                        }
                    }

                    maps[map_index].push(mapping);
                }
                None => break 'map_construction,
            }
        }
    }

    for (start_seed, range) in seed_ranges {
        // println!("---------------------------------- Seed {}", start_seed);
        let mut old_map_ranges = vec![(start_seed, range)];
        for map in &maps {
            // println!("------------ Map");
            let mut current_map_ranges = Vec::new();
            for map in map.to_vec() {
                // println!("---- Conversion");
                for range in &old_map_ranges {
                    if let Some(mapped_ranges) = map_range(*range, map) {
                        for mapped_range in mapped_ranges {
                            current_map_ranges = add_range_to_ranges(mapped_range, current_map_ranges);
                        }
                    }
                }
            }
            old_map_ranges = current_map_ranges;
        }
        // println!("{:?}", old_map_ranges);
        result = result.min(old_map_ranges.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap_or(&(u64::MAX, 0_u64)).0);
    }

    result.into()
}

fn map_range(range: (u64, u64), remapping: (u64, u64, u64)) -> Option<Vec<(u64, u64)>> {
    let (start, length) = range;
    let end = start + length;

    let mapped_start = std::cmp::max(std::cmp::min(remapping.1 as i64 + start as i64 - remapping.0 as i64, remapping.1 as i64 + remapping.2 as i64 - 1), remapping.1.try_into().unwrap());
    let mapped_end = std::cmp::max(std::cmp::min(remapping.1 as i64 + end as i64 - remapping.0 as i64, remapping.1 as i64 + remapping.2 as i64 - 1), remapping.1 as i64);

    if end <= remapping.0 {
        None
    } else if start >= remapping.0 + remapping.2 {
        None
    } else {
        // Range intersects with the remapping
        let mut result = Vec::new();

        if start < remapping.0 {
            result.push((start, remapping.0 - start));
        }

        if end > remapping.0 + remapping.2 {
            result.push(((mapped_start + remapping.0 as i64 - start as i64) as u64, mapped_end as u64));
            result.push((remapping.0 + remapping.2, end - (remapping.0 + remapping.2)));
        } else {
            result.push((mapped_start as u64, length));
        }

        Some(result)
    }
}

pub fn add_range_to_ranges(mut new_range: (u64, u64), ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    // print!("{:?} -> ||| {:?} ||| -> ", ranges, new_range);

    let mut new_ranges: Vec<(u64, u64)> = Vec::new();
    let mut inserted = false;

    for range in ranges {
        if new_range.0 + new_range.1 < range.0 {
            new_ranges.push(new_range);
            inserted = true;
        } else if new_range.0 <= range.0 + range.1 {
            new_range = (new_range.0.min(range.0), (new_range.0 + new_range.1).max(range.0 + range.1) - new_range.0.min(range.0));
        } else {
            new_ranges.push(range);
        }
    }

    if !inserted {
        new_ranges.push(new_range);
    }
    // println!("{:?}", new_ranges);
    return new_ranges;
}
