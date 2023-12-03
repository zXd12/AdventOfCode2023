use std::fs::File;
use std::io::{self, BufRead};

pub fn part_one(reader: io::BufReader<File>) -> io::Result<u32> {
    let mut result = 0;

    for line in reader.lines() {
        let line = line?;

        for c in line.chars() {
            if let Some(value) = c.to_digit(10) {
                result += value * 10;
                break;
            }
        }

        for c in line.chars().rev() {
            if let Some(value) = c.to_digit(10) {
                result += value;
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

        'left_to_right_enumeration: for (char_index, c) in line.chars().enumerate() {
            if let Some(value) = c.to_digit(10) {
                result += value * 10;
                break 'left_to_right_enumeration;
            } else if let Some(number_index) = numbers
                .iter()
                .position(|&number| line[char_index..].starts_with(number))
            {
                result += (number_index as u32 + 1) * 10;
                break 'left_to_right_enumeration;
            }
        }

        'right_to_left_enumeration: for (char_index, c) in line.chars().rev().enumerate() {
            if let Some(value) = c.to_digit(10) {
                result += value;
                break 'right_to_left_enumeration;
            } else if let Some(number_index) = numbers
                .iter()
                .position(|&number| line[..line.len() - char_index].ends_with(number))
            {
                result += number_index as u32 + 1;
                break 'right_to_left_enumeration;
            }
        }
    }

    Ok(result)
}

