use crate::common::{benchmark, InputParser};

pub fn run() {
    benchmark("01.1", &solve_first_part);
    benchmark("01.2", &solve_second_part);
}

fn solve_first_part() -> i64 {
    let values = include_str!("./input.txt").to_string().as_number_vector();

    let mut result = 0;

    for index in 1..values.len() {
        if values[index] > values[index-1] {
            result = result + 1;
        }
    }

    assert_eq!(result, 1154);

    return result;
}

fn solve_second_part() -> i64 {
    let values = include_str!("./input.txt").to_string().as_number_vector();

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

    assert_eq!(result, 1127);

    return result;
}