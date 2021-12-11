use std::{
    collections::{HashMap},
    vec,
};

use crate::common::benchmark;

pub fn run() {
    benchmark("10.1", &solve_first_part);
    benchmark("10.2", &solve_second_part);
}

fn solve_first_part() -> i64 {
    let mut parens_map = HashMap::new();

    parens_map.insert(')', '(');
    parens_map.insert(']', '[');
    parens_map.insert('}', '{');
    parens_map.insert('>', '<');

    let mut points = HashMap::new();

    points.insert(')', 3);
    points.insert(']', 57);
    points.insert('}', 1197);
    points.insert('>', 25137);

    let closings = [')', ']', '}', '>'];

    let rows = include_str!("./input.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut illegal_characters: Vec<char> = vec![];

    for row in rows {
        let mut stacc: Vec<char> = vec![];

        'row: for char in row {
            if closings.contains(&char) {
                let popped = stacc.pop().unwrap();

                if popped != parens_map[&char] {
                    illegal_characters.push(char);
                    break 'row;
                }
            } else {
                stacc.push(char);
            }
        }
    }

    let result = illegal_characters.iter().fold(0, |acc, char| {
        acc + points[char]
    });

    assert_eq!(result, 399153);

    return result as i64;
}

fn solve_second_part() -> i64 {
    let mut closing_to_opening_map = HashMap::new();

    closing_to_opening_map.insert(')', '(');
    closing_to_opening_map.insert(']', '[');
    closing_to_opening_map.insert('}', '{');
    closing_to_opening_map.insert('>', '<');

    let mut opening_to_closing_map = HashMap::new();

    opening_to_closing_map.insert('(', ')');
    opening_to_closing_map.insert('[', ']');
    opening_to_closing_map.insert('{', '}');
    opening_to_closing_map.insert('<', '>');

    let mut points = HashMap::new();

    points.insert(')', 1);
    points.insert(']', 2);
    points.insert('}', 3);
    points.insert('>', 4);

    let closings = [')', ']', '}', '>'];

    let rows = include_str!("./input.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut results: Vec<i64> = vec!();

    'row: for row in rows {
        let mut stacc: Vec<char> = vec![];

        for char in &row {
            if closings.contains(&char) {
                let popped = stacc.pop().unwrap();

                if popped != closing_to_opening_map[&char] {
                    // from part 1 - find illegal strings and dismiss them
                    continue 'row;
                }
            } else {
                stacc.push(*char);
            }
        }

        let mut remainder: Vec<char> = vec!();

        while !stacc.is_empty() {
            let char = stacc.pop().unwrap();
        
            let matching_char = opening_to_closing_map[&char];
            
            remainder.push(matching_char);
        }

        results.push(
            calculate_points_for_line(&remainder, &points)
        );
    }

    results.sort();

    let middle_result_index = (results.len() - 1) / 2;

    let result = *results.get(middle_result_index).unwrap();

    assert_eq!(result, 2995077699);

    return result as i64;
}

fn calculate_points_for_line(remainder: &Vec<char>, points: &HashMap<char, i64>) -> i64 {
    let mut result = 0;

    for char in remainder {
        result = (result * 5) + points[char];
    }

    return result;
}