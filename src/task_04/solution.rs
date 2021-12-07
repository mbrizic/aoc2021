use std::{vec};

use crate::common::{benchmark};

type Row = Vec<i32>;

struct Matrix {
    rows: Vec<Row>
}

pub fn run() {
    benchmark("04.1", &solve_first_part);
    benchmark("04.2", &solve_second_part);
}

fn solve_first_part() -> i64 {

    let input_segments = include_str!("./input.txt")
        .split("\n\n")
        .collect::<Vec<&str>>();

    let numbers: Vec<i32> = input_segments[0]
        .split(",")
        .map(|num| 
            num.parse::<i32>().unwrap()
        )
        .collect();

    let mut drawn_numbers: Vec<i32> = vec!();
    let matrices: Vec<Matrix> = construct_matrices(&input_segments);

    // push first five numbers at the start
    for i in 0..4 {
        drawn_numbers.push(
            numbers[i]
        );
    }

    let winning_matrix_index = draw_numbers_until_first_win(&matrices, &numbers, &mut drawn_numbers).unwrap();

    let result = calculate_final_score(
    &matrices[winning_matrix_index],
            &drawn_numbers
    );

    assert_eq!(result, 23177);

    return result;
}

fn solve_second_part() -> i64 {

    let input_segments = include_str!("./input.txt")
        .split("\n\n")
        .collect::<Vec<&str>>();

    let numbers: Vec<i32> = input_segments[0]
        .split(",")
        .map(|num| 
            num.parse::<i32>().unwrap()
        )
        .collect();

    let mut drawn_numbers: Vec<i32> = vec!();
    let matrices: Vec<Matrix> = construct_matrices(&input_segments);

    // push first five numbers at the start
    for i in 0..4 {
        drawn_numbers.push(
            numbers[i]
        );
    }

    let winning_matrix_index = draw_numbers_until_all_win(&matrices, &numbers, &mut drawn_numbers).unwrap();

    let result = calculate_final_score(
    &matrices[winning_matrix_index],
            &drawn_numbers
    );

    assert_eq!(result, 6804);

    return result;
}

fn draw_numbers_until_first_win(matrices: &Vec<Matrix>, numbers: &Vec<i32>, drawn_numbers: &mut Vec<i32>) -> Result<usize, bool> {
    for i in 4..numbers.len() {
        let num = numbers[i];
        
        drawn_numbers.push(num);

        for (matrix_index, matrix) in matrices.iter().enumerate() {
            let valid_result = verify_matrix_valid(&matrix, &drawn_numbers);
    
            if valid_result.is_ok() {
                return Ok(matrix_index);
            }
        }
    }

    return Err(false);
}

fn draw_numbers_until_all_win(matrices: &Vec<Matrix>, numbers: &Vec<i32>, drawn_numbers: &mut Vec<i32>) -> Result<usize, bool> {
    let mut winning_matrices: Vec<usize> = vec!();

    for i in 4..numbers.len() {
        let num = numbers[i];
        
        drawn_numbers.push(num);

        for (matrix_index, matrix) in matrices.iter().enumerate() {
            let valid_result = verify_matrix_valid(&matrix, &drawn_numbers);
    
            if valid_result.is_ok() && !winning_matrices.contains(&matrix_index) {
                winning_matrices.push(matrix_index);

                if winning_matrices.len() == matrices.len() {
                    return Ok(matrix_index);
                }
            }
        }
    }

    return Err(false);
}

fn construct_matrices(input_segments: &Vec<&str>) -> Vec<Matrix> {
    let mut matrices: Vec<Matrix> = vec!();

    for i in 1..input_segments.len() {
        let rows = input_segments[i]
            .lines()
            .map(|line| 
                line
                    .split_whitespace()
                    .map(|char| 
                        char.trim().parse::<i32>().unwrap()
                    )
                    .collect::<Row>()
            )
            .collect::<Vec<Row>>();

        let matrix = Matrix {
            rows: rows
        };

        matrices.push(matrix);
    }

    return matrices;
}

fn calculate_final_score(matrix: &Matrix, drawn_numbers: &Vec<i32>) -> i64 {
    let mut sum = 0;

    for row in matrix.rows.iter() {
        for num in row.iter() {
            if !drawn_numbers.contains(num) {
                sum += num;
            }
        }
    }

    let last_drawn_number = drawn_numbers.last().unwrap();

    return (last_drawn_number * sum) as i64;
}

fn verify_matrix_valid(matrix: &Matrix, numbers: &Vec<i32>) -> Result<(), bool> {

    for row in matrix.rows.iter() {
        'row1: for (num_index, num) in row.iter().enumerate() {
            if numbers.contains(num) {
                if num_index == 4 {
                    return Ok(());
                }
            } else {
                break 'row1;
            }
        }
    }
    
    for column_index in 0..5 {
        'row2: for (row_index, row) in matrix.rows.iter().enumerate() {
            let number = row.get(column_index).unwrap();

            if numbers.contains(number) {
                if row_index == 4 {
                    return Ok(());
                }
            } else {
                break 'row2;
            }
        }
    }

    return Err(false);
}