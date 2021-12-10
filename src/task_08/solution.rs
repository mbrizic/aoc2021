use crate::common::{benchmark};

pub fn run() {
    benchmark("08.1", &solve_first_part);
    benchmark("08.2", &solve_second_part);
}

struct Row {
    inputs: Vec<String>,
    outputs: Vec<String>
}

fn solve_first_part() -> i64 {
 
    let rows = include_str!("./input.txt")
        .lines()
        .map(|l| {
            let segments: Vec<&str> = l.split(" | ").collect();
            let outputs = segments.get(1).unwrap();

            return outputs.split(" ").collect::<Vec<&str>>();
        })
        .collect::<Vec<Vec<&str>>>();

    let mut result = 0;

    for row in rows {
        for digit in row {
            // check for easy numbers which have unique numbers of segments
            if digit.len() == 2 || digit.len() == 4 || digit.len() == 3 || digit.len() == 7 {
                result += 1;
            }
        }
    }

    assert_eq!(result, 519);

    return result as i64;
}

/**
 * Some performance improvements were done, however bulk of the perf cost comes from 
 * those string sorting and comparison operations, so those should at least get memoized.
 * 
 * It can also be improved by changing the algorithm to work in a different way.
 */
fn solve_second_part() -> i64 {
 
    let rows = include_str!("./input.txt")
        .lines()
        .map(|l| {
            let segments: Vec<&str> = l.split(" | ").collect();
            let inputs = segments.get(0).unwrap();
            let outputs = segments.get(1).unwrap();

            return Row {
                inputs: inputs
                    .split(" ")
                    .map(|d| sort_digits(d))
                    .collect::<Vec<String>>(),
                outputs: outputs
                    .split(" ")
                    .map(|d| sort_digits(d))
                    .collect::<Vec<String>>(),
            };
        })
        .collect::<Vec<Row>>();


    let mut result = 0;

    for row in rows {

        let mut digit_segments: [String; 10] = [
            String::from(""),           // 0
            String::from(""),           // 1
            String::from(""),           // 2
            String::from(""),           // 3
            String::from(""),           // 4
            String::from(""),           // 5
            String::from(""),           // 6
            String::from(""),           // 7
            String::from("abcdefg"),    // 8
            String::from(""),           // 9
        ];

        let complicated_inputs: Vec<&String> = row.inputs.iter().filter(|input|
            input.len() == 5 || input.len() == 6
        ).collect();

        for code in &row.inputs {
            parse_easy_numbers(&mut digit_segments, code);
        }

        loop {
            for code in &complicated_inputs {
                parse_complicated_numbers(&mut digit_segments, code);
            }

            if digit_segments.iter().all(|s| !s.is_empty()) {
                break;
            }
        }

        let row_result = row.outputs.iter().map(|c| {
            return digit_segments.iter().enumerate()
                .find(|(index, code)| (*code).eq(c))
                .unwrap()
                .0
                .to_string();
        })
        .collect::<Vec<String>>()
        .join("");

        let row_result: i32 = row_result.parse::<i32>().unwrap();

        result += row_result;
    }

    assert_eq!(result, 1027483);

    let result = result;

    return result as i64;
}

fn parse_complicated_numbers(map: &mut [String; 10], code: &str) {
    let top_right = find_difference_between_two_strings(
        map[8].as_str(), 
        map[6].as_str()
    );

    if code.len() == 6 {
        if !check_if_codes_overlap(code, map[7].as_str()) {
            map[6] = code.to_string();
        } else if !check_if_codes_overlap(code, map[4].as_str()) {
            map[0] = code.to_string();
        } else if !map[6].is_empty() && !map[0].is_empty() {
            map[9] = code.to_string();
        }
    } else if code.len() == 5 {
        if check_if_codes_overlap(code, map[1].as_str()) {
            map[3] = code.to_string();
        } else if !top_right.is_empty() && check_if_codes_overlap(code, &top_right) {
            map[2] = code.to_string();
        } else if !map[3].is_empty() && !map[2].is_empty() {
            map[5] = code.to_string();
        }
    }
}

fn parse_easy_numbers(map: &mut [String; 10], code: &str) {
    if code.len() == 2 {
        map[1] = code.to_string();
    } else if code.len() == 4 {
        map[4] = code.to_string();
    } else if code.len() == 3 {
        map[7] = code.to_string();
    }
}

// Add some memoization here
fn sort_digits(code: &str) -> String {
    let mut split = code
        .split("")
        .collect::<Vec<&str>>();

    split.sort();

    let joined = split.join("");

    return joined;
}

fn find_difference_between_two_strings(bigger: &str, smaller: &str) -> String {
    if smaller.len() == 0 {
        return String::from("");
    }

    if bigger == smaller {
        return String::from("");
    }

    let smaller_chars: Vec<char> = smaller.chars().collect();

    let contains: String = bigger.chars().filter(|char|
        !smaller_chars.contains(&char)
    ).collect();

    return contains;
}

fn check_if_codes_overlap(bigger: &str, smaller: &str) -> bool {
    if smaller.len() == 0 {
        return false;
    }

    let bigger_chars: Vec<char> = bigger.chars().collect();

    let contains = smaller.chars().all(|char|
        bigger_chars.contains(&char)
    );

    return contains;
}