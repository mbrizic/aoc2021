use std::{vec};

use crate::common::{benchmark};

pub fn run() {
    benchmark("06.1", &solve_first_part);
    benchmark("06.2", &solve_second_part);
}

/**
 * This is already pretty performant as it's using counter sums
 * and not storing entire array of all the fish anymore, however it can
 * still be improved in some more mathy way, by calculating result directly,
 * without even using iteration.
 * **/
fn solve_first_part() -> i64 {

    let fishes = include_str!("./input.txt")
        .split(",")
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut fish_counts = vec![0; 9];

    for fish in fishes {
        fish_counts[fish as usize] = fish_counts[fish as usize] + 1;
    }

    for i in 1..=80 {
        fish_counts = update_fish_counts(&fish_counts);
        // println!("After day {}\t {:?}", i, fish_counts);
    }

    let result = fish_counts.iter().fold(0, |acc, &g| acc + g);

    assert_eq!(result, 360610);

    return result;
}

fn solve_second_part() -> i64 {

    let fishes = include_str!("./input.txt")
        .split(",")
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut fish_counts: Vec<i64> = vec![0; 9];

    for fish in fishes {
        fish_counts[fish as usize] = fish_counts[fish as usize] + 1;
    }

    for i in 1..=256 {
        fish_counts = update_fish_counts(&fish_counts);
    }

    let result = fish_counts.iter().fold(0, |acc, &g| acc + g);

    assert_eq!(result, 1631629590423);

    return result;
}

fn update_fish_counts(groups: &Vec<i64>) -> Vec<i64> {
    let mut new_fish_count = 0;

    let mut new_iteration = groups
        .iter()
        .enumerate()
        .map(|(group_index, count)| {
            if group_index == 0 {
                new_fish_count += count;
            }

            if group_index == 8 {
                return groups[group_index]; // not important, we're overriding this later
            }

            return groups[group_index + 1];
        })
        .collect::<Vec<i64>>();

    new_iteration[6] = new_iteration[6] + new_fish_count;
    new_iteration[8] = new_fish_count;

    return new_iteration;
}
