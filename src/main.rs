mod questions;
mod solver;

const CURRENT_DAY: u32 = 7;

fn main() {
    
    println!("Day 7 question 1 : {}", solver::solve(7, 1, None));
    println!("Day 7 question 2 : {}", solver::solve(7, 2, None));
    println!("Day 7 question 1 perfs : {}μs avg", solver::perf_test(7, 1, None, 1e3 as u32));
    println!("Day 7 question 2 perfs : {}μs avg", solver::perf_test(7, 2, None, 1e3 as u32));
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