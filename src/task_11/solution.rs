use std::{
    collections::{HashMap},
    vec,
};

use crate::common::benchmark;

static ROWS: usize = 10;
static COLS: usize = 10;

pub fn run() {
    benchmark("11.1", &solve_first_part);
    // benchmark("11.2", &solve_second_part);
}

fn solve_first_part() -> i64 {
    let mut rows = include_str!("./test.txt")
        .lines()
        .map(|line| line.chars()
            .map(|a| a.to_string().parse::<i32>().unwrap())
            .collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();

    let result = 1;

    for i in 0..2 {
        println!("Iteration {}", i);

        rows = iterate(&rows);
    }

    // assert_eq!(result, 399153);

    return result as i64;
}

fn iterate(rows: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let row_size = rows[0].len();

    let mut new_state = vec!(
        vec!(0; row_size); 
        rows.len()
    );

    let mut exploded = new_state.clone();

    for (row_index, row) in rows.iter().enumerate() {
        for (col_index, num) in row.iter().enumerate() {
            let new_value = num + 1;

            if new_value == 9 {
                flash_around(&mut new_state, &mut exploded, row_index, col_index);
                // ret[row_index][col_index] = 0;
            } else {
                new_state[row_index][col_index] = new_value;
            }
        }
        println!("{:?}", new_state[row_index]);
    }


    return new_state;
}

fn flash_around(rows: &mut Vec<Vec<i32>>, exploded: &mut Vec<Vec<i32>>, row_index: usize, col_index: usize) {
    exploded[row_index][col_index] = 1;

    rows[row_index][col_index] = 0;    

    if row_index > 0 && row_index < ROWS {
        flash_around(rows, exploded, row_index + 1, col_index)
    }
    if row_index > 0 && row_index < ROWS {
        flash_around(rows, exploded, row_index - 1, col_index)
    }
}

