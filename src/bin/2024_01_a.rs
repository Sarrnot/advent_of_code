use std::{env, fs};

/// Solution for https://adventofcode.com/2024/day/1 - Part One.
/// Run by `cargo run --bin 2024_01_a`. (optionally can provide custom file path by adding ` -- ./custom_file_path/file.txt`)
fn main() -> Result<(), String> {
    // Get file path
    let args = env::args().collect();
    let file_path = match args_to_file_path(args) {
        Some(value) => value,
        None => "./resources/2024_01_a-data.txt".to_string(),
    };

    // Read file
    let file_content = match fs::read_to_string(file_path) {
        Ok(text) => text,
        Err(err) => return Err(err.to_string()),
    };

    // Parse lines
    let mut lists = match parse_file(file_content) {
        Ok(value) => value,
        Err(err) => return Err(err),
    };

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

fn parse_file(text: String) -> Result<[Vec<i32>; 2], String> {
    let mut lists = [Vec::new(), Vec::new()];

    // Iterate over each line in the file.
    for (i, line) in text.lines().enumerate() {
        // Split values by whitespace
        let parts = line.split_whitespace();
        let line_num = i + 1;

        // Validate - has 2 values separated by whitespace
        if parts.clone().count() != 2 {
            return Err(format!(
                "Error on line {line_num}. Must have 2 values separated by whitespace."
            ));
        }

        // Parse and save both values on current line.
        for (j, raw_value) in parts.enumerate() {
            match raw_value.parse::<i32>() {
                Ok(value) => {
                    lists[j].push(value);
                }
                Err(err) => {
                    return Err(format!(
                        "Invalid value \"{raw_value}\" on line {line_num}. {err}"
                    ));
                }
            }
        }
    }

    Ok(lists)
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
