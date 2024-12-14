use std::{collections::HashMap, usize};

use advent_of_code::get_lists_from_file;

const FILE_PATH: &str = "./resources/2024_01_b.txt";

/// Solution for https://adventofcode.com/2024/day/1 - Part Two.
/// Run by `cargo run --bin 2024_01_b`.
fn main() -> Result<(), String> {
    // Parse file
    let [left_list, right_list]: [Vec<i32>; 2] = get_lists_from_file(FILE_PATH)?;

    // Create a map from right_list [item value => occurrence count]
    let mut right_count_map: HashMap<i32, usize> = HashMap::new();
    for item in right_list {
        match right_count_map.get_mut(&item) {
            Some(count) => {
                *count += 1;
            }
            None => {
                right_count_map.insert(item, 1);
            }
        };
    }

    // Calculate similarity score
    let mut similarity_score = 0;
    for item in left_list {
        match right_count_map.get(&item) {
            Some(count) => {
                let i32_count = i32::try_from(*count)
                    .expect("Aborting, unhandled edge case. Count overflowed i32.");
                similarity_score += item * i32_count;
            }
            None => (),
        }
    }

    // Print result
    println!("Similarity score is: {}", similarity_score);

    Ok(())
}
