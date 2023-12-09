pub fn part_one(input: &str) -> i128 {
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

pub fn part_two(input: &str) -> i128 {
    let numbers = [
        "one".as_bytes(),
        "two".as_bytes(),
        "three".as_bytes(),
        "four".as_bytes(),
        "five".as_bytes(),
        "six".as_bytes(),
        "seven".as_bytes(),
        "eight".as_bytes(),
        "nine".as_bytes(),
    ];

    let mut result = 0;

    for line in input.lines() {
        let bytes = line.as_bytes();
        for (char_index, byte) in bytes.iter().enumerate() {
            if b'0' <= *byte && *byte <= b'9' {
                result += (byte - b'0') as u32 * 10;
                break;
            } else if let Some(number_index) = numbers
                .iter()
                .position(|&number| bytes[char_index..].starts_with(number))
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
                .position(|&number| bytes[..line.len() - char_index].ends_with(number))
            {
                result += number_index as u32 + 1;
                break;
            }
        }
    }

    result.into()
}
