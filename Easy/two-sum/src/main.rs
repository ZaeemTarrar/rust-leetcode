use std::time::Instant;

mod solutions;
use crate::solutions::sol1::sol1::two_sum;

fn run() {
    let inputs: [(Vec<i32>, i32); 3] = [
        (vec![2, 7, 11, 15], 9),
        (vec![3, 2, 4], 6),
        (vec![3, 3], 6),
    ];
    for (k, v) in inputs.iter().enumerate() {
        let result = two_sum(v.0.clone(), v.1);
        println!("Result[{}]: {:?}", k, result);
    }
}

fn run_time() {
    let start_time = Instant::now();
    run();
    let end_time = Instant::now();
    let duration = end_time - start_time;
    println!("\nRun Duration: [{:?}]\n", duration);
}

fn multi_run(cycles: u32) {
    for i in 0..cycles {
        println!("\n<=== Cycle: [{}] ===>\n", i + 1);
        run();
    }
}

fn multi_run_time(cycles: u32) {
    let start_time = Instant::now();
    multi_run(cycles);
    let end_time = Instant::now();
    let duration = end_time - start_time;
    println!("\nRun Duration: [{:?}]\n", duration);
}

fn main() {
    // run_time();
    multi_run_time(10_00_000); // 1 million
}
