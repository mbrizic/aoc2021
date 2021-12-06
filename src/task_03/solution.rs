use std::{ops::Range, time::Instant, usize};

use crate::common::print_results;

pub fn run() {
    solve_first_part();
    solve_second_part();
}

/**
 * Solved in a naive way, performance could be improved
 */
fn solve_first_part() {
    let now = Instant::now();

    let lines = include_str!("./input.txt").lines()
        .collect::<Vec<&str>>();

    let param_length = 12;
    let treshold = *&lines.len() as i32 / 2;
    let mut gamma_map: Vec<i32> = vec![0; param_length];

    for i in 0..param_length {
        for line in &lines {
            let char = line.chars().nth(i).unwrap();

            if char == '1' {
                gamma_map[i] += 1;

                if gamma_map[i] >= treshold {
                    break;
                }
            }
        }
    }

    let mut gamma_str = "".to_string();

    for val in &gamma_map {
        if *val >= treshold {
            gamma_str.push_str("1");
        } else {
            gamma_str.push_str("0");
        }
    }

    let binary_base: i32 = 2;
    let limit = binary_base.pow(12) - 1;

    let gamma = binary_string_to_number(gamma_str);
    let epsilon = limit - gamma;

    let result = gamma * epsilon;

    print_results("03.1", result, now.elapsed());

    assert_eq!(result, 3429254);
}

/**
 * Resorted to recursion as it looked cleaner.
 * Might be interesting to compare performance using iteration instead.
 */
fn solve_second_part() {
    let now = Instant::now();

    let lines = include_str!("./input.txt").lines().collect::<Vec<&str>>();

    let char_index: usize = 0;
    let indices_under_check: Vec<usize> = Range {
        start: 0,
        end: lines.len(),
    }
    .collect();

    let oxygen_index = find(
        &lines,
        &indices_under_check,
        char_index,
        &oxygen_comparison_function,
    );
    let oxygen = binary_string_to_number(lines[oxygen_index].to_string());

    let co2_index = find(
        &lines,
        &indices_under_check,
        char_index,
        &co2_comparison_function,
    );
    let co2 = binary_string_to_number(lines[co2_index].to_string());

    let result = oxygen * co2;

    print_results("03.2", result, now.elapsed());

    assert_eq!(result, 5410338);
}

fn find(
    data: &Vec<&str>,
    indices: &Vec<usize>,
    char_index_under_check: usize,
    comparison: &dyn Fn(usize, usize) -> bool,
) -> usize {
    if indices.len() == 1 {
        let index = indices.first().unwrap();

        return *index;
    }

    let mut positive_indices: Vec<usize> = vec![];
    let mut negative_indices: Vec<usize> = vec![];

    indices.into_iter().for_each(|index| {
        let line = data.get(*index).unwrap();

        if line.chars().nth(char_index_under_check).unwrap() == '1' {
            positive_indices.push(*index);
        } else {
            negative_indices.push(*index);
        }
    });

    if comparison(negative_indices.len(), positive_indices.len()) {
        return find(
            data,
            &negative_indices.clone(),
            char_index_under_check + 1,
            &comparison,
        );
    } else {
        return find(
            data,
            &positive_indices.clone(),
            char_index_under_check + 1,
            &comparison,
        );
    }
}

fn oxygen_comparison_function(positive: usize, negative: usize) -> bool {
    positive > negative
}

fn co2_comparison_function(positive: usize, negative: usize) -> bool {
    positive <= negative
}

fn binary_string_to_number(binary: String) -> i32 {
    let mut dec: u64 = 0;
    for (i, bit) in binary.trim().chars().rev().enumerate() {
        let bit_num = bit.to_digit(10).expect("Error while parsing bit");

        dec += (bit_num * u32::pow(2, i as u32)) as u64;
    }

    return dec as i32;
}
