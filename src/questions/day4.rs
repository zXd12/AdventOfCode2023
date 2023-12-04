use std::fs::File;
use std::io::{self, BufRead};

pub fn part_one(reader: io::BufReader<File>) -> io::Result<u32> {
    let mut result = 0;

    for line in reader.lines() {
        let line = line?;

        let mut card_score = 1;

        let mut line_parts = line.split([':', '|'].as_ref());
        let winning_numbers = line_parts.nth(1).unwrap().trim().split_ascii_whitespace().collect::<Vec<&str>>();

        for number in line_parts.next().unwrap().trim().split_ascii_whitespace() {
            if winning_numbers.contains(&number) {
                card_score <<= 1;
            }
        }

        result += card_score >> 1;
    }

    Ok(result)
}

pub fn part_two(reader: io::BufReader<File>) -> io::Result<u32> {
    let mut result = 0;
    let mut card_duplicates = vec![1];

    for (i, line) in reader.lines().enumerate() {
        let line = line?;

        let mut card_score = 0;

        let mut line_parts = line.split([':', '|'].as_ref());
        let winning_numbers = line_parts.nth(1).unwrap().trim().split_ascii_whitespace().collect::<Vec<&str>>();

        for number in line_parts.next().unwrap().trim().split_ascii_whitespace() {
            if winning_numbers.contains(&number) {
                card_score += 1;
            }
        }

        // resize the vector if needed
        // only needed because we don't know the line count
        if card_duplicates.len() < i+card_score+1 {
            card_duplicates.resize(i+card_score+1, 1);
        }

        for j in 1..card_score+1 {
            card_duplicates[i+j] += card_duplicates[i];
        };

        result += card_duplicates[i];
    }

    Ok(result)
}
