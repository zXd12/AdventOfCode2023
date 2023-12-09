pub fn part_one(input: &str) -> i128 {
    let mut result = 0;

    'line_loop: for (line_number, line) in input.lines().enumerate() {
        let bytes = line.as_bytes();
        let mut index: usize = 5; // skips "Game "

        loop {
            if bytes[index] == b':' {
                break;
            }
            index += 1;
        }

        index += 2;

        loop {
            let mut value: u32 = (bytes[index] - b'0') as u32;
            loop {
                index += 1;
                if bytes[index] == b' ' {
                    break;
                }
                value *= 10;
                value += (bytes[index] - b'0') as u32;
            }

            index += 1;

            match bytes[index] {
                b'r' => {
                    if value > 12 {
                        continue 'line_loop;
                    }
                    index += 5;
                }
                b'g' => {
                    if value > 13 {
                        continue 'line_loop;
                    }
                    index += 7;
                }
                b'b' => {
                    if value > 14 {
                        continue 'line_loop;
                    }
                    index += 6;
                }
                _ => panic!("Invalid color name"),
            }

            if index > bytes.len() - 1 {
                result += line_number as u32 + 1;
                continue 'line_loop;
            }
        }
    }
    result.into()
}

pub fn part_two(input: &str) -> i128 {
    let mut result = 0;

    'line_loop: for line in input.lines() {
        let bytes = line.as_bytes();
        let mut index: usize = 5; // skips "Game "
        let mut max_amounts = (0, 0, 0);

        loop {
            if bytes[index] == b':' {
                break;
            }
            index += 1;
        }

        index += 2;

        loop {
            let mut value: u32 = (bytes[index] - b'0') as u32;
            loop {
                index += 1;
                if bytes[index] == b' ' {
                    break;
                }
                value *= 10;
                value += (bytes[index] - b'0') as u32;
            }

            index += 1;

            match bytes[index] {
                b'r' => {
                    max_amounts.0 = max_amounts.0.max(value);
                    index += 5;
                }
                b'g' => {
                    max_amounts.1 = max_amounts.1.max(value);
                    index += 7;
                }
                b'b' => {
                    max_amounts.2 = max_amounts.2.max(value);
                    index += 6;
                }
                _ => panic!("Invalid color name"),
            }

            if index > bytes.len() - 1 {
                result += max_amounts.0 * max_amounts.1 * max_amounts.2;
                continue 'line_loop;
            }
        }
    }
    result.into()
}
