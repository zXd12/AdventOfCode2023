use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub(crate) fn part_one(path: &Path) -> io::Result<u32> {

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut result = 0;

    for line in reader.lines() {
        
        let line = line?;

        for char in line.chars() {
            if char.is_numeric() {
                result += char.to_digit(10).unwrap_or_default() * 10;
                break
            }
        }

        for char in line.chars().rev() {
            if char.is_numeric() {
                result += char.to_digit(10).unwrap_or_default();
                break
            }
        }
    }

    Ok(result)
}

pub(crate) fn part_two(path: &Path) -> io::Result<u32> {
    
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut result = 0;

    for line in reader.lines() {

        let line = line?;
        // print!("{} : ", line);

        'left_to_right_enumeration: for (char_index, char) in line.chars().enumerate() {
            if char.is_numeric() {
                result += char.to_digit(10).unwrap_or_default() * 10;
                // print!("{}", char.to_digit(10).unwrap_or_default());
                break 'left_to_right_enumeration
            } else {
                for (number_index, number) in numbers.iter().enumerate() {
                    if line[char_index..].starts_with(number) {
                        result += (number_index as u32 + 1) * 10;
                        // print!("{}", number_index as u32 + 1);
                        break 'left_to_right_enumeration
                    }
                }
            }
        }

        'right_to_left_enumeration: for (char_index, char) in line.chars().rev().enumerate() {
            if char.is_numeric() {
                result += char.to_digit(10).unwrap_or_default();
                // println!("{}", char.to_digit(10).unwrap_or_default());
                break 'right_to_left_enumeration
            } else {
                for (number_index, number) in numbers.iter().enumerate() {
                    if line[..line.len() - char_index].ends_with(number) {
                        result += number_index as u32 + 1;
                        // println!("{}", number_index as u32 + 1);
                        break 'right_to_left_enumeration
                    }
                }
            }
        }
    }

    Ok(result)
}