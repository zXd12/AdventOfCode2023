mod questions;
mod solver;

const CURRENT_DAY: u32 = 4;

fn main() {

    for day in 1..=CURRENT_DAY {
        for question in 1..=2 {
            println!("Day {} question {} : {}", day, question, solver::solve(day, question, None));
        }
    }

    // perfs(1e5 as u32);    
}



fn perfs(iterations: u32) {

    for day in 1..=CURRENT_DAY {
        for question in 1..=2 {
            println!("Day {} question {} perftest 1m iterations : {:.4?}ns avg", day, question, solver::perf_test(day, question, None, iterations));
        }
    }
}