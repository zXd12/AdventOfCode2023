use std::fs::File;
use std::io::{self, BufRead};

pub fn part_one(reader: io::BufReader<File>) -> io::Result<u32> {
    let mut result = 0;

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        let pulls = line.split([';', ':'].as_ref()).collect::<Vec<&str>>()[1..].to_vec();

        let mut is_valid = true;

        'analyse_game: for pull in &pulls {
            let colors: Vec<&str> = pull.split([','].as_ref()).collect();
            for color in colors {
                let values: Vec<&str> = color.split([' '].as_ref()).collect();
                let count = values[1].parse::<u32>().unwrap();
                let color_name = values[2];
                is_valid &= match color_name {
                    "red" => count <= 12,
                    "green" => count <= 13,
                    "blue" => count <= 14,
                    _ => true,
                };
                if !is_valid {
                    break 'analyse_game;
                }
            }
        }
        if is_valid {
            result += index as u32 + 1;
        }
    }

    Ok(result)
}

pub fn part_two(reader: io::BufReader<File>) -> io::Result<u32> {
    let mut result = 0;

    for line in reader.lines() {
        let line = line?;
        let pulls = line.split([';', ':'].as_ref()).collect::<Vec<&str>>()[1..].to_vec();

        let mut colors_min = (0, 0, 0);

        for pull in &pulls {
            let colors: Vec<&str> = pull.split([','].as_ref()).collect();
            for color in colors {
                let values: Vec<&str> = color.split([' '].as_ref()).collect();
                let count = values[1].parse::<u32>().unwrap();
                let color_name = values[2];
                match color_name {
                    "red" => colors_min.0 = colors_min.0.max(count),
                    "green" => colors_min.1 = colors_min.1.max(count),
                    "blue" => colors_min.2 = colors_min.2.max(count),
                    _ => panic!("Unvalid color name"),
                };
            }
        }

        result += colors_min.0 * colors_min.1 * colors_min.2;
    }

    Ok(result)
}
