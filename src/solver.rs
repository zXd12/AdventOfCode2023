use std::{
    fs::File,
    io::{self},
    time::Instant,
};

use crate::questions::*;

pub fn solve(day: u32, question: u32, path: Option<&str>) -> u32 {
    let default_path = &format!("inputs/input{}.txt", day);
    let input_path = path.unwrap_or_else(|| default_path);
    let file = File::open(input_path).unwrap();
    let input = io::BufReader::new(file);
    return get_function(day, question)(input).unwrap();
}

pub fn perf_test(day: u32, question: u32, path: Option<&str>, iterations: u32) -> u32 {
    let default_path = &format!("inputs/input{}.txt", day);
    let input_path = path.unwrap_or_else(|| default_path);
    let file = File::open(input_path).unwrap();
    let fun = get_function(day, question);
    let mut data: Vec<u32> = vec![];

    // warmup
    let reader = io::BufReader::new(file.try_clone().unwrap());
    let _ = fun(reader);

    for _ in 0..iterations {
        let reader = io::BufReader::new(file.try_clone().unwrap());
        let start_time = Instant::now();
        let _ = fun(reader);
        let execution_time = Instant::now() - start_time;
        let nanos = execution_time.subsec_nanos();
        data.push(nanos);
    }

    // let min_time = *data.iter().min().unwrap();
    // let max_time = *data.iter().max().unwrap();
    let total_time = data.iter().sum::<u32>();
    let avg = total_time.checked_div(data.len() as u32).unwrap();

    avg
}

fn get_function(day: u32, question: u32) -> fn(io::BufReader<File>) -> Result<u32, io::Error> {
    match (day, question) {
        (1, 1) => day1::part_one,
        (1, 2) => day1::part_two,
        (2, 1) => day2::part_one,
        (2, 2) => day2::part_two,
        (3, 1) => day3::part_one,
        (3, 2) => day3::part_two,
        (4, 1) => day4::part_one,
        (4, 2) => day4::part_two,
        _ => panic!("Unsuported day or question!"),
    }
}
