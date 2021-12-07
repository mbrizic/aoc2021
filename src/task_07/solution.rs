use std::{vec, convert::TryInto};

use crate::common::{benchmark};

pub fn run() {
    benchmark("07.1", &solve_first_part);
    benchmark("07.2", &solve_second_part);
}

fn solve_first_part() -> i64 {
    let crabs = include_str!("./input.txt")
        .split(",")
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let calculate_fuel = &calculate_fuel_part_1;

    let starting_position = calculate_good_starting_position(&crabs);
    let fuel_costs = [];

    let fuel_cost_at_average_position = calculate_fuel(
        &crabs, 
        &fuel_costs, 
        starting_position
    );

    let result = search_for_optimal_position(
        &crabs, 
        &fuel_costs, 
        &calculate_fuel,
        starting_position as i32, 
        fuel_cost_at_average_position
    );

    assert_eq!(result, 349812);

    return result as i64;
}

fn solve_second_part() -> i64 {
    let crabs = include_str!("./input.txt")
        .split(",")
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let calculate_fuel = &calculate_fuel_part_2;

    let starting_position = calculate_good_starting_position(&crabs);
    let fuel_costs = build_fuel_costs_map();

    let fuel_cost_at_average_position = calculate_fuel(
        &crabs, 
        &fuel_costs, 
        starting_position
    );

    let result = search_for_optimal_position(
        &crabs, 
        &fuel_costs,
        &calculate_fuel,
        starting_position as i32, 
        fuel_cost_at_average_position
    );

    assert_eq!(result, 99763899);

    return result as i64;
}

fn build_fuel_costs_map() -> [i32; 1500] {
    let mut current_fuel_cost = 0;

    return (0..1500).map(|index| {
        current_fuel_cost += index;

        return current_fuel_cost;
    }).collect::<Vec<i32>>()
    .try_into().expect("Mismatched size");
}

fn calculate_good_starting_position(crabs: &Vec<i32>) -> i32 {
    let sum = crabs.iter()
        .fold(0, |acc, crab| acc + crab) as f32;
    let average_position = (sum / crabs.len() as f32).round() as i32;

    return average_position;
}

fn search_for_optimal_position(
    crabs: &Vec<i32>,
    fuel_costs: &[i32],
    calculate_fuel: &dyn Fn(&Vec<i32>, &[i32], i32) -> i32,
    position: i32, 
    cost_here: i32
) -> i32 {
    let cost_for_next_position = calculate_fuel(&crabs, &fuel_costs, position + 1);
    let cost_for_prev_position = calculate_fuel(&crabs, &fuel_costs, position - 1);

    // println!("Cost at position {} is {}, + {}, - {}", position, cost_here, cost_for_next_position, cost_for_prev_position);

    if cost_here < cost_for_next_position && cost_here < cost_for_prev_position {
        return cost_here;
    } else if cost_for_next_position < cost_here {
        return search_for_optimal_position(
            &crabs, 
            &fuel_costs, 
            &calculate_fuel,
            position + 1, 
            cost_for_next_position
        );
    } else if cost_for_prev_position < cost_here {
        return search_for_optimal_position(
            &crabs, 
            &fuel_costs, 
            &calculate_fuel,
            position - 1, 
            cost_for_prev_position
        );
    } else {
        panic!("Oops, shouldn't land here")
    }

}

fn calculate_fuel_part_1(
    crabs: &Vec<i32>, 
    fuel_costs: &[i32], 
    index: i32
) -> i32 {
    return crabs.iter()
        .fold(0, |acc, crab| {
            acc + (crab - index).abs()
        });
}

fn calculate_fuel_part_2(
    crabs: &Vec<i32>, 
    fuel_costs: &[i32], 
    index: i32
) -> i32 {
    return crabs.iter()
        .fold(0, |acc, crab| {
            let required_moves_count = (crab - index).abs();
            let fuel_cost = fuel_costs[required_moves_count as usize];
            return acc + fuel_cost;
        });
}