use std::env;

use advent_of_code::get_columns_from_file;

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
    let mut columns: [Vec<i32>; 2] = get_columns_from_file(&file_path)?;

    // Sort columns
    for column in columns.iter_mut() {
        column.sort();
    }

    // Calculate and print total distance
    let distance_sum = calculate_distance_sum(columns);
    println!("Total distance is {}", distance_sum);

    Ok(())
}

fn args_to_file_path(args: Vec<String>) -> Option<String> {
    if args.len() < 2 {
        return None;
    }

    Some(args[1].clone())
}

fn calculate_distance_sum(sorted_columns: [Vec<i32>; 2]) -> i32 {
    let [column1, column2] = sorted_columns;
    let mut distance_sum = 0;

    for (i, value1) in column1.into_iter().enumerate() {
        let value2 = column2[i]; // no need to check whether exists - already validated while parsing file
        distance_sum += (value1 - value2).abs();
    }

    distance_sum
}
