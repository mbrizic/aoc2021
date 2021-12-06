use std::{time::Instant, vec};

use crate::common::print_results;

pub fn run() {
    solve_first_part();
    solve_second_part();
}

struct Position {
    x: i32,
    y: i32,
}

fn solve_first_part() {
    solve_and_verify("05.1", false, 5585);
}

fn solve_second_part() {
    solve_and_verify("05.2", true, 17193);
}

/**
 * No time spent optimizing this in any way, come back later to try to speed it up.
 **/
fn solve_and_verify(task_number: &str, should_calculate_diagonals: bool, expected_result: i32) {
    let now = Instant::now();

    let lines = include_str!("./input.txt")
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|pos| {
                    let nums = pos
                        .split(",")
                        .map(|n| n.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();

                    return Position {
                        x: nums[0],
                        y: nums[1],
                    };
                })
                .collect::<Vec<Position>>()
        })
        .collect::<Vec<Vec<Position>>>();

    let grid_size = 1000;
    let mut points: Vec<Vec<i32>> = vec![vec!(0; grid_size); grid_size];

    for line in lines {
        let start = &line[0];
        let end = &line[1];

        if start.x == end.x {
            // travel horizontally

            let range = if start.y >= end.y {
                end.y..=start.y
            } else {
                start.y..=end.y
            };

            for y in range {
                let cell = points
                    .get_mut(y as usize)
                    .unwrap()
                    .get_mut(start.x as usize)
                    .unwrap();

                *cell += 1;
            }
        } else if start.y == end.y {
            // travel vertically

            let range = if start.x >= end.x {
                end.x..=start.x
            } else {
                start.x..=end.x
            };

            for x in range {
                let cell = points
                    .get_mut(start.y as usize)
                    .unwrap()
                    .get_mut(x as usize)
                    .unwrap();

                *cell += 1;
            }
        } else if should_calculate_diagonals {
            let number_of_moves = (start.x - end.x).abs() + 1;

            let should_increase_x = end.x > start.x;
            let should_increase_y = end.y > start.y;

            let mut x = start.x;
            let mut y = start.y;

            for i in 0..number_of_moves {
                let cell = points
                    .get_mut(y as usize)
                    .unwrap()
                    .get_mut(x as usize)
                    .unwrap();

                *cell += 1;

                x = if should_increase_x { x + 1 } else { x - 1 };

                y = if should_increase_y { y + 1 } else { y - 1 };
            }
        }
    }

    let mut result = 0;

    for (i, row) in points.iter().enumerate() {
        for cell in row {
            if *cell > 1 {
                result += 1;
            }
        }
    }

    print_results(task_number, result, now.elapsed());

    assert_eq!(result, expected_result);
}
