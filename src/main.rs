mod questions;
mod solver;

const CURRENT_DAY: u32 = 10;

fn main() {
    
    // let _ = day5::add_range_to_ranges((5,5), vec![(4,3), (9,3)]);

    println!("Day 11 question 1 : {}", solver::solve(11, 1, None));
    println!("Day 11 question 2 : {}", solver::solve(11, 2, None));
    // println!("Day 10 question 2 : {}", solver::solve(10, 2, Some("inputs/input10_2.txt")));
    // println!("Day 10 question 1 perfs : {}μs avg", solver::perf_test(10, 1, None, 1e3 as u32));
    // println!("Day 10 question 2 perfs : {}μs avg", solver::perf_test(10, 2, None, 1e3 as u32));
    // solve_all();
    // perfs(1e3 as u32);    
}

fn solve_all() {
    for day in 1..=CURRENT_DAY {
        for question in 1..=2 {
            println!("Day {} question {} : {}", day, question, solver::solve(day, question, None));
        }
    }
}

fn perfs(iterations: u32) {

    for day in 1..=CURRENT_DAY {
        for question in 1..=2 {
            println!("Day {} question {} perftest {:e} iterations : {:.4?}μs avg", day, question, iterations, solver::perf_test(day, question, None, iterations));
        }
    }
}