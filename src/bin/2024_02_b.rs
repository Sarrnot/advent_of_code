use std::cmp::Ordering;

use advent_of_code::get_rows_from_file;

const FILE_PATH: &str = "./resources/2024_02.txt";

/// Solution for https://adventofcode.com/2024/day/2 - Part Two.
/// Run by `cargo run --bin 2024_02_b`.
fn main() -> Result<(), String> {
    // Parse file
    let rows: Vec<Vec<i32>> = get_rows_from_file(FILE_PATH)?;
    let mut safe_count: usize = 0;

    // Validate each row. valid => increment safe_count
    for row in rows {
        // Skip (count as invalid) row with 0 or 1 entries (not specified whether un/safe in the rules)
        if row.len() <= 1 {
            continue;
        }

        if validate_row(&row) {
            safe_count += 1;
            continue;
        }

        // A very naive and unoptimized solution.
        // Might be interesting to explore solution that would allow a customizable `tolerance` level and was optimized at the same time. (tried it, gets suprisingly quite complex)
        for i in 0..row.len() {
            let mut row_slice = row.clone();
            row_slice.remove(i);
            if validate_row(&row_slice) {
                safe_count += 1;
                break;
            }
        }
    }

    // Print result
    println!("Number of safe reports: {}", safe_count);

    Ok(())
}

fn validate_row(row: &Vec<i32>) -> bool {
    // Determine asc/desc trend based on first two values.
    let descending = match row[0].cmp(&row[1]) {
        Ordering::Equal => return false, // "adjacent levels differ by at least one"
        Ordering::Greater => true,
        Ordering::Less => false,
    };

    // Validate rules for row values.
    for i in 0..(row.len() - 1) {
        let val1 = row[i];
        let val2 = row[i + 1];

        // Validate asc/desc trend - rule #1 "The levels are either all increasing or all decreasing".
        match val1.cmp(&val2) {
            Ordering::Equal => return false, // "adjacent levels differ by at least one"
            Ordering::Greater => {
                if !descending {
                    return false;
                }
            }
            Ordering::Less => {
                if descending {
                    return false;
                }
            }
        }

        // Validate Δ - rule #2 "Any two adjacent levels differ by at least one and at most three".
        let values_delta = (val1 - val2).abs();

        if values_delta > 3 {
            return false;
        }
        // equality already checked in previous rule
    }

    true
}
