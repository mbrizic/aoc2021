use std::time::Instant;

use crate::common::print_results;

pub fn run() {
    solve_first_part();
    solve_second_part();
}

fn solve_first_part() {
    let now = Instant::now();

    let values = include_str!("./input.txt").lines();

    let mut hor = 0;
    let mut dep = 0;

    values
        .map(|line| {
            let mut segments = line.split_whitespace();
            return (
                segments.next().unwrap(),
                segments.next().unwrap().parse::<i32>().unwrap(),
            );
        })
        .for_each(|a| match a.0 {
            "forward" => hor += a.1,
            "down" => dep += a.1,
            "up" => dep -= a.1,
            _ => panic!(),
        });

    let result = hor * dep;

    print_results("02.1", result, now.elapsed());

    assert_eq!(result, 2117664);
}

fn solve_second_part() {
    let now = Instant::now();

    let values = include_str!("./input.txt").lines();

    let mut hor = 0;
    let mut dep = 0;
    let mut aim = 0;

    values
        .map(|line| {
            let mut segments = line.split_whitespace();
            return (
                segments.next().unwrap(),
                segments.next().unwrap().parse::<i32>().unwrap(),
            );
        })
        .for_each(|a| match a.0 {
            "forward" => {
                hor += a.1;
                dep += aim * a.1;
            }
            "down" => aim += a.1,
            "up" => aim -= a.1,
            _ => panic!(),
        });

    let result = hor * dep;

    print_results("02.2", result, now.elapsed());

    assert_eq!(result, 2073416724);
}
