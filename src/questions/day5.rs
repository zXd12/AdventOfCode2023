fn check_mapping(map: (u64, u64, u64), value: u64) -> Option<u64> {
    if value >= map.0 && value < map.0 + map.2 - 1 {
        Some(map.1 + (value - map.0))
    } else {
        None
    }
}

fn translate(mappings: Vec<(u64, u64, u64)>, value: u64) -> u64 {
    let mut result = value;

    for map in mappings {
        match check_mapping(map, result) {
            Some(new_value) => {
                result = new_value;
                break;
            }
            None => {}
        }
    }
    result
}

pub fn part_one(input: &str) -> u64 {
    let mut lines = input.lines();
    let mut result = u64::MAX;
    let mut maps = Vec::new();
    let seeds = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    lines.next();

    let mut new_translation_map = Vec::new();

    'map_construction: loop {
        lines.next();

        loop {
            let line = lines.next();
            match line {
                Some(line) => {
                    if line.is_empty() {
                        break;
                    }
                    let translation_line: Vec<&str> = line.split_ascii_whitespace().collect();
                    new_translation_map.push((
                        translation_line[1].parse::<u64>().unwrap(),
                        translation_line[0].parse::<u64>().unwrap(),
                        translation_line[2].parse::<u64>().unwrap(),
                    ));
                }
                None => break 'map_construction,
            }
        }

        maps.push(new_translation_map);

        new_translation_map = Vec::new();
    }

    maps.push(new_translation_map);
    for mut seed in seeds {
        for map in &maps {
            seed = translate(map.to_vec(), seed);
        }
        result = result.min(seed.try_into().unwrap());
    }

    result
}

/*
pub fn part_two(input: &str) -> u64 {
    let mut lines = input.lines();
    let mut result = u64::MAX;

    let mut maps = Vec::new();

    let mut seeds_ranges: Vec<(u64, u64)> = vec![];
    let binding = lines.next().unwrap();
    let mut seed_line = binding.split_ascii_whitespace().skip(1);
    loop {
        match seed_line.next() {
            Some(seed) => {
                let seed = seed.parse::<u64>().unwrap();
                let seed2 = seed_line.next().unwrap().parse::<u64>().unwrap();
                seeds_ranges.push((seed, seed2));
            }
            None => break,
        };
    }

    lines.next();

    let mut new_translation_map = TranslationMap::new(String::from(
        lines.next().unwrap().split(':').next().unwrap(),
    ));

    'map_construction: loop {
        loop {
            let line = lines.next();
            match line {
                Some(line) => {
                    let line = line;
                    if line.is_empty() {
                        break;
                    }

                    let translation_line: Vec<&str> = line.split_ascii_whitespace().collect();
                    new_translation_map.add_translation(Translation::new(
                        translation_line[1].parse::<u64>().unwrap(),
                        translation_line[0].parse::<u64>().unwrap(),
                        translation_line[2].parse::<u64>().unwrap(),
                    ));
                }
                None => break 'map_construction,
            }
        }
        maps.push(new_translation_map);

        new_translation_map = TranslationMap::new(String::from(
            lines.next().unwrap().split(':').next().unwrap(),
        ))
    }

    maps.push(new_translation_map);
    for seed_range in seeds_ranges {
        println!(
            "seed range: {:?}, seed count: {}",
            seed_range,
            seed_range.1 - 1
        );
        for (i, mut seed) in (seed_range.0..seed_range.0 + seed_range.1).enumerate() {
            // update the loading bar
            if i % 1000 == 0 {
                print!("\r");
                print!("{}%", (i as f32 / seed_range.1 as f32 * 100.0) as u32);
            }
            for map in &maps {
                seed = map.translate(seed);
            }
            result = result.min(seed.try_into().unwrap());
        }
    }

    result
}
*/