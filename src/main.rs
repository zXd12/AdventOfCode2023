mod questions;
mod solver;

fn main() {

    println!("Day 1 question 1 : {}", solver::solve(1, 1, None));
    println!("Day 1 question 2 : {}", solver::solve(1, 2, None));
    println!("Day 2 question 1 : {}", solver::solve(2, 1, None));
    println!("Day 2 question 2 : {}", solver::solve(2, 2, None));
    println!("Day 3 question 1 : {}", solver::solve(3, 1, None));
    println!("Day 3 question 2 : {}", solver::solve(3, 2, None));

    // perf_graphs();    
}

fn perf_graphs() {
    println!("Day 1 question 1 perftest 1m iterations : {:.4?}ns avg", solver::perf_test(1, 1, None, 1e6 as u32));
    println!("Day 1 question 2 perftest 1m iterations : {:.4?}ns avg", solver::perf_test(1, 2, None, 1e6 as u32));

    println!("Day 2 question 1 perftest 1m iterations : {:.4?}ns avg", solver::perf_test(2, 1, None, 1e6 as u32));
    println!("Day 2 question 2 perftest 1m iterations : {:.4?}ns avg", solver::perf_test(2, 2, None, 1e6 as u32));

    println!("Day 3 question 1 perftest 1m iterations : {:.4?}ns avg", solver::perf_test(3, 1, None, 1e6 as u32));
    println!("Day 3 question 2 perftest 1m iterations : {:.4?}ns avg", solver::perf_test(3, 2, None, 1e6 as u32));
}