use std::env;

use advent_of_code::get_lists_from_file;

/// Solution for https://adventofcode.com/2024/day/1 - Part One.
/// Run by `cargo run --bin 2024_01_a`. (optionally can provide custom file path by adding ` -- ./custom_file_path/file.txt`)
fn main() -> Result<(), String> {
    // Get file path
    let args = env::args().collect();
    let file_path = match args_to_file_path(args) {
        Some(value) => value,
        None => "./resources/2024_01_a.txt".to_string(),
    };

    // Parse file
    let mut lists: [Vec<i32>; 2] = get_lists_from_file(&file_path)?;

    // Sort lists
    for list in lists.iter_mut() {
        list.sort();
    }

    // Calculate and print total distance
    let distance_sum = calculate_distance_sum(lists);
    println!("Total distance is {}", distance_sum);

    Ok(())
}

fn args_to_file_path(args: Vec<String>) -> Option<String> {
    if args.len() < 2 {
        return None;
    }

    Some(args[1].clone())
}

fn calculate_distance_sum(sorted_lists: [Vec<i32>; 2]) -> i32 {
    let [list1, list2] = sorted_lists;
    let mut distance_sum = 0;

    for (i, value1) in list1.into_iter().enumerate() {
        let value2 = list2[i]; // no need to check whether exists - already validated while parsing file
        distance_sum += (value1 - value2).abs();
    }

    distance_sum
}
