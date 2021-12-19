use std::{
    collections::{HashMap, HashSet},
    vec,
};

use crate::common::{benchmark, InputParser};

pub fn run() {
    benchmark("09.1", &solve_first_part);
    // benchmark("09.2", &solve_second_part);
}

#[derive(Debug)]
struct Point {
    row: usize,
    column: usize,
    value: i32,
}

static RISK_LEVEL: i32 = 1;

fn solve_first_part() -> i64 {
    let rows = include_str!("./input.txt")
        .to_string().as_number_matrix();

    let potential_points = determine_potential_low_points(&rows);

    let low_points = potential_points.iter().filter(|point| {
        if point.column > 0 && point.value >= get_value_at_point(&rows, point.row, point.column - 1)
        {
            return false;
        }

        if point.row < rows.len() - 1
            && point.value >= get_value_at_point(&rows, point.row + 1, point.column)
        {
            return false;
        }

        if point.row > 0 && point.value >= get_value_at_point(&rows, point.row - 1, point.column) {
            return false;
        }

        return true;
    });

    let result = low_points.fold(0, |acc, point| {
        return acc + point.value + RISK_LEVEL;
    });

    assert_eq!(result, 572);

    return result as i64;
}

// fn solve_second_part() -> i64 {
    // let rows = include_str!("./input.txt")
    // .to_string().as_number_matrix();

//     let potential_points = determine_potential_low_points(&rows);

//     let row_size = rows.get(0).unwrap().len();

//     let low_points = potential_points
//         .iter()
//         .filter(|point| {
//             if point.column > 0
//                 && point.value >= get_value_at_point(&rows, point.row, point.column - 1)
//             {
//                 return false;
//             }

//             if point.row < rows.len() - 1
//                 && point.value >= get_value_at_point(&rows, point.row + 1, point.column)
//             {
//                 return false;
//             }

//             if point.row > 0
//                 && point.value >= get_value_at_point(&rows, point.row - 1, point.column)
//             {
//                 return false;
//             }

//             return true;
//         })
//         .map(|point| {
//             let basin_size = calculate_basin_size(&rows, row_size, &point.row, $point.column);

//             return Point {
//                 row: point.row,
//                 column: point.column,
//                 value: basin_size
//             }
//         });

//     let result = low_points.fold(0, |acc, point| {
//         println!("{:?}", point);
//         return acc + point.value;
//     });

//     assert_eq!(result, 572);

//     return result as i64;
// }

fn calculate_basin_size(rows: &Vec<Vec<i32>>, row_size: i32, row_index: usize, column_index: usize) -> i32 {

    let mut res = 1;

    // if point.column < row_size && get_value_at_point(&rows, point.row, point.column + 1) < 9 {
    //     res += 1;
    // }

    // if point.column > 0 && get_value_at_point(&rows, point.row, point.column - 1) < 9 {
    //     res += 1;
    // }

    // if row_index < rows.len() - 1 && get_value_at_point(&rows, row_index + 1, column_index) < 9
    // {
    //     res += 1;
    // }

    // if point.row > 0 && get_value_at_point(&rows, point.row - 1, point.column) < 9 {
    //     res += 1;
    // }

    return res;
}

fn get_value_at_point(rows: &Vec<Vec<i32>>, row: usize, column: usize) -> i32 {
    let row_size = rows.get(0).unwrap().len();

    if row >= rows.len() {
        return 9; // if no value there, return max value
    }

    if column >= row_size {
        return 9; // if no value there, return max value
    }

    let row = rows.get(row).unwrap().get(column).unwrap();

    return *row;
}

fn determine_potential_low_points(rows: &Vec<Vec<i32>>) -> Vec<Point> {
    let mut potential_points: Vec<Point> = vec![];

    for (row_index, row) in rows.iter().enumerate() {
        for column_index in 0..(row.len() - 1) {
            if row[column_index] < row[column_index + 1] {
                potential_points.push(Point {
                    row: row_index,
                    column: column_index,
                    value: row[column_index],
                });
            }
        }

        // check last item by comparing with previous one
        let last_item = row[row.len() - 1];
        let second_to_last_item = row[row.len() - 2];

        if last_item < second_to_last_item {
            potential_points.push(Point {
                row: row_index,
                column: row.len() - 1,
                value: last_item,
            });
        }
    }

    return potential_points;
}
