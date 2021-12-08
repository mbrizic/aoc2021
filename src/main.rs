use std::time::Instant;

mod task_01;
mod task_02;
mod task_03;
mod task_04;
mod task_05;
mod task_06;
mod task_07;
mod task_08;

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
    task_07::solution::run();
    task_08::solution::run();

    println!("\n> Total: {:.2?}", now.elapsed());
}
