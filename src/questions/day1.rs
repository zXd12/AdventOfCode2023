use std::fs::File;
use std::io::{self, BufRead};

pub fn part_one(reader: io::BufReader<File>) -> io::Result<u32> {
    let mut result = 0;

    for line in reader.lines() {
        let line = line?;

        for char in line.chars() {
            if char.is_numeric() {
                result += char.to_digit(10).unwrap_or_default() * 10;
                break;
            }
        }

        for char in line.chars().rev() {
            if char.is_numeric() {
                result += char.to_digit(10).unwrap_or_default();
                break;
            }
        }
    }

    Ok(result)
}

pub fn part_two(reader: io::BufReader<File>) -> io::Result<u32> {
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut result = 0;

    for line in reader.lines() {
        let line = line?;

        'left_to_right_enumeration: for (char_index, char) in line.chars().enumerate() {
            if char.is_numeric() {
                result += char.to_digit(10).unwrap_or_default() * 10;
                break 'left_to_right_enumeration;
            } else {
                for (number_index, number) in numbers.iter().enumerate() {
                    if line[char_index..].starts_with(number) {
                        result += (number_index as u32 + 1) * 10;
                        break 'left_to_right_enumeration;
                    }
                }
            }
        }

        'right_to_left_enumeration: for (char_index, char) in line.chars().rev().enumerate() {
            if char.is_numeric() {
                result += char.to_digit(10).unwrap_or_default();
                break 'right_to_left_enumeration;
            } else {
                for (number_index, number) in numbers.iter().enumerate() {
                    if line[..line.len() - char_index].ends_with(number) {
                        result += number_index as u32 + 1;
                        break 'right_to_left_enumeration;
                    }
                }
            }
        }
    }

    Ok(result)
}
