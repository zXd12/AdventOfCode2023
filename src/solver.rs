use std::{
    fs::File,
    io::Read,
    time::Instant,
};

use crate::questions::*;

pub fn solve(day: u32, question: u32, path: Option<&str>) -> u64 {
    let default_path = &format!("inputs/input{}.txt", day);
    let input_path = path.unwrap_or_else(|| default_path);

    let mut input = String::new();
    let _ = File::open(input_path).unwrap().read_to_string(&mut input);

    return get_function(day, question)(&input);
}

pub fn perf_test(day: u32, question: u32, path: Option<&str>, iterations: u32) -> u32 {
    let default_path = &format!("inputs/input{}.txt", day);
    let input_path = path.unwrap_or_else(|| default_path);
    let fun = get_function(day, question);
    let mut times: Vec<u32> = vec![];

    let mut input = String::new();
    let _ = File::open(input_path).unwrap().read_to_string(&mut input);
    
    for _ in 0..iterations {
        let start_time = Instant::now();
        let _ = fun(&input);
        let execution_time = Instant::now() - start_time;
        let micros = execution_time.subsec_micros();
        times.push(micros);
    }

    // let min_time = *data.iter().min().unwrap();
    // let max_time = *data.iter().max().unwrap();
    let total_time = times.iter().sum::<u32>();
    let avg = total_time.checked_div(times.len() as u32).unwrap();

    avg
}

fn get_function(day: u32, question: u32) -> fn(&str) -> u64 {
    match (day, question) {
        (1, 1) => day1::part_one,
        (1, 2) => day1::part_two,
        (2, 1) => day2::part_one,
        (2, 2) => day2::part_two,
        (3, 1) => day3::part_one,
        (3, 2) => day3::part_two,
        (4, 1) => day4::part_one,
        (4, 2) => day4::part_two,
        (5, 1) => day5::part_one,
        (5, 2) => day5::part_two,
        (6, 1) => day6::part_one,
        (6, 2) => day6::part_two,
        (7, 1) => day7::part_one,
        (7, 2) => day7::part_two,
        (8, 1) => day8::part_one,
        (8, 2) => day8::part_two,
        _ => panic!("Unsuported day or question!"),
    }
}