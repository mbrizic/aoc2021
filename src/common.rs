use std::time::Duration;
use std::time::Instant;

pub trait InputParser {
    fn as_number_matrix(self) -> Vec<Vec<i32>>;
    fn as_number_vector(self) -> Vec<i32>;
}

impl InputParser for String {
    fn as_number_matrix(self) -> Vec<Vec<i32>> {
        self.lines()
            .map(|line| {
                line.chars()
                    .map(|a| a.to_string().parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>()
    }

    fn as_number_vector(self) -> Vec<i32> {
        self.lines()
            .map(|a| a.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    }
}

pub fn print_results(task_number: &str, result: &String, time: Duration) {
    println!("| {}\t| {:.2?}\t| {}", task_number, time, result);
}

pub fn benchmark(task_number: &str, function: &dyn Fn() -> i64) {
    let now = Instant::now();

    let result = function();

    println!("| {}\t| {:.2?}\t| {}", task_number, now.elapsed(), result);
}
