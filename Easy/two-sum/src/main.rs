use std::time::Instant;

mod solutions;
use crate::solutions::sol1::sol1;
use crate::solutions::sol2::sol2;
use crate::solutions::sol3::sol3;

fn run(task: fn(nums: Vec<i32>, target: i32) -> Vec<i32>) {
    let inputs: [(Vec<i32>, i32); 3] = [
        (vec![2, 7, 11, 15], 9),
        (vec![3, 2, 4], 6),
        (vec![3, 3], 6),
    ];
    for (k, v) in inputs.iter().enumerate() {
        let result = task(v.0.clone(), v.1);
        println!("Result[{}]: {:?}", k, result);
    }
}

fn run_time(task: fn(nums: Vec<i32>, target: i32) -> Vec<i32>) -> String {
    let start_time = Instant::now();
    run(task);
    let end_time = Instant::now();
    let duration = end_time - start_time;
    return format!("\nRun Duration: [{:?}]\n", duration);
}

fn multi_run(cycles: u32, task: fn(nums: Vec<i32>, target: i32) -> Vec<i32>) {
    for i in 0..cycles {
        // println!("\n<=== Cycle: [{}] ===>\n", i + 1);
        run(task);
    }
}

fn multi_run_time(cycles: u32, task: fn(nums: Vec<i32>, target: i32) -> Vec<i32>) -> String {
    let start_time = Instant::now();
    multi_run(cycles, task);
    let end_time = Instant::now();
    let duration = end_time - start_time;
    return format!("\nMulti-Run Duration: [{:?}]\n", duration);
}

fn main() {
    let cycles = 10_00_000; // 1 million
    // Solution 1
    println!("{}", run_time(sol1::two_sum));
    // println!("{}", multi_run_time(cycles, sol1::two_sum));

    // Solution 2
    println!("{}", run_time(sol2::two_sum));
    // println!("{}", multi_run_time(cycles, sol2::two_sum));

    // Solution 3
    println!("{}", run_time(sol3::two_sum));
    // println!("{}", multi_run_time(cycles, sol3::two_sum));
}
