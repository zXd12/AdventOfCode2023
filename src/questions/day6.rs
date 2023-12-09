use std::f64;

fn line_to_vec(line: String) -> Vec<u64> {
    line.split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

fn line_to_single_u64(line: String) -> u64 {
    line.split_ascii_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap()
}

fn get_bound(time: u64, distance: u64) -> u64 {
    ((((time as f64).powi(2) - 4.0 * distance as f64).sqrt() + time as f64) / 2.0 - 1e-5) as u64 * 2
        + 1
        - time
}

pub fn part_one(input: &str) -> i128 {

    let mut lines= input.lines();

    let times = line_to_vec(lines.next().unwrap().to_string());
    let distances = line_to_vec(lines.next().unwrap().to_string());

    times
        .iter()
        .zip(distances.iter())
        .map(|(&time, &distance)| get_bound(time, distance))
        .product::<u64>().into()
}

pub fn part_two(input: &str) -> i128 {

    let mut lines= input.lines();

    get_bound(
        line_to_single_u64(lines.next().unwrap().to_string()),
        line_to_single_u64(lines.next().unwrap().to_string()),
    ).into()
}