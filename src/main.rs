use std::time::Instant;

use crate::common::print_results;

mod task_01;
mod task_02;
mod task_03;
mod task_04;
mod task_05;
mod task_06;

pub mod common;

fn main() {

    println!("| Task \t| Time \t\t| Answer");
    println!("|-------|---------------|---------------");

    let now = Instant::now();
    
    task_01::solution::run();
    task_02::solution::run();
    task_03::solution::run();
    task_04::solution::run();
    task_05::solution::run();
    task_06::solution::run();

    // println!("| Total | {:.2?}", now.elapsed());
}
