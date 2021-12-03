mod task_01;
mod task_02;
mod task_03;

pub mod common;

fn main() {

    println!("| Task \t| Time \t\t| Answer");
    println!("|-------|---------------|---------------");

    task_01::solution::run();
    task_02::solution::run();
    task_03::solution::run();
}

