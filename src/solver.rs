use std::{fs::File, io};

use crate::questions::*;

pub fn solve(day: u32, question: u32) -> u32 {
    let input_path = &format!("inputs/input{}.txt", day);
    let file = File::open(input_path).unwrap();
    let reader = io::BufReader::new(file);
    return match (day, question) {
        (1, 1) => day1::part_one(reader),
        (1, 2) => day1::part_two(reader),
        (2, 1) => day2::part_one(reader),
        (2, 2) => day2::part_two(reader),
        _ => panic!("Unsuported day or question!"),
    }
    .unwrap();
}
