use std::path::Path;

mod questions;

fn main() {
    println!("Hello, world!");
    println!("Question 1 part 1 : {}", questions::question1::part_one(Path::new("inputs/input1_1.txt")).unwrap());
    println!("Question 1 part 2 : {}", questions::question1::part_two(Path::new("inputs/input1_2.txt")).unwrap());
}
