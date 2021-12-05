use std::{ops::Range, time::Instant, usize};

use crate::common::print_results;

pub fn run() {
    solve_first_part();
    // solve_second_part();
}

fn solve_first_part() {
    let now = Instant::now();

    let segments = include_str!(".\\test.txt")
        .split("\n\n")
        .collect::<Vec<&str>>();

    let numbers = segments[0];

    for i in 1..segments.len() {
        println!("{}", i);
    }

    let result = 1;

    print_results("04.1", result, now.elapsed());

    // assert_eq!(result, 3429254);
}

fn solve_second_part() {
    let now = Instant::now();

    let result = 1;

    print_results("04.2", result, now.elapsed());

    // assert_eq!(result, 5410338);
}
