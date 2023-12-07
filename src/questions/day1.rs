pub fn part_one(input: &str) -> u64 {
    let mut result = 0;

    for line in input.lines() {

        for byte in line.as_bytes() {
            if b'0' <= *byte && *byte <= b'9' {
                result += (byte - 48) as u32 * 10;
                break;
            }
        }

        for byte in line.as_bytes().iter().rev() {
            if b'0' <= *byte && *byte <= b'9' {
                result += (*byte - b'0') as u32;
                break;
            }
        }
    }

    result.into()
}

pub fn part_two(input: &str) -> u64 {
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut result = 0;

    for line in input.lines() {

        for (char_index, byte) in line.as_bytes().iter().enumerate() {
            if b'0' <= *byte && *byte <= b'9' {
                result += (byte - b'0') as u32 * 10;
                break;
            } else if let Some(number_index) = numbers
                .iter()
                .position(|&number| line[char_index..].starts_with(number))
            {
                result += (number_index as u32 + 1) * 10;
                break;
            }
        }

        for (char_index, byte) in line.as_bytes().iter().rev().enumerate() {
            if b'0' <= *byte && *byte <= b'9' {
                result += (byte - b'0') as u32;
                break;
            } else if let Some(number_index) = numbers
                .iter()
                .position(|&number| line[..line.len() - char_index].ends_with(number))
            {
                result += number_index as u32 + 1;
                break;
            }
        }
    }

    result.into()
}

