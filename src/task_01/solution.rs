use std::time::Instant;

use crate::common::print_results;

pub fn run() {
    solve_first_part();
    solve_second_part();
}

fn solve_first_part() {
    let now = Instant::now();

    let values = include_str!("./input.txt").lines()
            .map(|a| a.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

    let mut result = 0;

    for index in 1..values.len() {
        if values[index] > values[index-1] {
            result = result + 1;
        }
    }

    print_results("01.1", &result.to_string(), now.elapsed());

    assert_eq!(result, 1154);
}

fn solve_second_part() {
    let now = Instant::now();

    let values = include_str!("./input.txt").lines()
            .map(|a| a.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

    // Functional approach, equally performant
    // let result = values
    //         .windows(4)
    //         .filter(|x| x[3] > x[0])
    //         .count();

    let mut result = 0;

    for i in 1..values.len()-2 {

        if values[i + 2] > values[i - 1] {
            result = result + 1;
        }

    }

    print_results("01.2", &result.to_string(), now.elapsed());

    assert_eq!(result, 1127);
}