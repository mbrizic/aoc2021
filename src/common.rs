use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::time::Duration;

pub fn read_file(path: &str) -> std::io::Result<Vec<i32>> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let lines = contents
        .lines()
        .map(|a| a.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    return Ok(lines);
}

pub fn print_results(task_number: &str, result: i32, time: Duration) {
    println!("| {} \t| {} \t\t| {:.2?} \t|", task_number, result, time);
}