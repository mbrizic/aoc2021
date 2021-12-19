use std::{
    collections::{HashMap},
    vec,
};

use crate::common::benchmark;
use crate::common::InputParser;

static ROWS_COUNT: usize = 10;
static ROW_LENGHT: usize = 10;
static OCTOPUS_FLASHED_CODE: i32 = 0;

struct Octopus {
    col: usize,
    row: usize,
}

pub fn run() {
    benchmark("11.1", &solve_first_part);
    benchmark("11.2", &solve_second_part);
}

fn solve_first_part() -> i64 {
    let mut rows = include_str!("./input.txt").to_string().as_number_matrix();

    let mut result = 0;

    for i in 0..100 {
        // println!("Iteration {}", i+1);

        let res = increase_energy_levels_for_all(&rows);

        rows = res.0;
        let octopuses_to_flash = res.1;

        start_flashing(&mut rows, &octopuses_to_flash);

        for row in &rows {
            for char in row {
                if *char == 0 {
                    result += 1;
                }
            }
        }
    }

    assert_eq!(result, 1694);

    return result as i64;
}

fn solve_second_part() -> i64 {
    let mut rows = include_str!("./input.txt").to_string().as_number_matrix();

    let mut result = 0;

    for i in 0..500 {
        let res = increase_energy_levels_for_all(&rows);

        rows = res.0;
        let octopuses_to_flash = res.1;

        start_flashing(&mut rows, &octopuses_to_flash);

        let is_all = rows.iter().all(|r| {
            r.iter().all(|char| *char == 0)
        });

        if is_all {
            result = i + 1;
            break;
        }
    }

    assert_eq!(result, 346);

    return result as i64;
}

fn increase_energy_levels_for_all(rows: &Vec<Vec<i32>>) -> (Vec<Vec<i32>>, Vec<Octopus>) {
    let mut new_state = vec!(
        vec!(0; ROW_LENGHT); 
        ROWS_COUNT
    );

    let mut octopuses_to_flash: Vec<Octopus> = vec!();

    for (row_index, row) in rows.iter().enumerate() {
        for (col_index, num) in row.iter().enumerate() {
            let new_value = num + 1;

            new_state[row_index][col_index] = new_value;

            if new_value > 9 {
                octopuses_to_flash.push(Octopus {
                    col: col_index,
                    row: row_index
                })
            }
        }
    }

    return (new_state, octopuses_to_flash);
}

fn start_flashing(rows: &mut Vec<Vec<i32>>, flashables: &Vec<Octopus>) {

    for octo in flashables {
        try_flash_around_octo(rows, octo.row, octo.col);
    }
   
    for octopus in flashables {
        rows[octopus.row][octopus.col] = 0;
   }
}

fn try_flash_around_octo(rows: &mut Vec<Vec<i32>>, row: usize, col: usize) {
    let is_already_flashed = rows[row][col] == OCTOPUS_FLASHED_CODE;

    if !is_already_flashed {
        rows[row][col] += 1;
    }

    // return if not flashable or if already flashed
    if rows[row][col] <= 9 || is_already_flashed {
        return;
    }

    rows[row][col] = OCTOPUS_FLASHED_CODE;

    // bump 3 elements above the point
    if row > 0 {
        try_flash_around_octo(rows, row - 1, col);
        
        if col > 0 {
            try_flash_around_octo(rows, row - 1, col - 1);
        }
        if col < ROW_LENGHT - 1 {
            try_flash_around_octo(rows, row - 1, col + 1);
        }
    }

    if col > 0 {
        try_flash_around_octo(rows, row, col - 1);
    }
    if col < ROW_LENGHT - 1 {
        try_flash_around_octo(rows, row, col + 1);
    }

    // bump 3 elements below
    if row < ROWS_COUNT - 1{
        try_flash_around_octo(rows, row + 1, col);

        if col > 0 {
            try_flash_around_octo(rows, row + 1, col - 1);
        }

        if col < ROW_LENGHT - 1{
            try_flash_around_octo(rows, row + 1, col + 1);
        }
    }

}