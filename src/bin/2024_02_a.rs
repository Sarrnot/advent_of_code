use std::cmp::Ordering;

use advent_of_code::get_rows_from_file;

const FILE_PATH: &str = "./resources/2024_02.txt";

/// Solution for https://adventofcode.com/2024/day/2 - Part One.
/// Run by `cargo run --bin 2024_02_a`.
fn main() -> Result<(), String> {
    // Parse file
    let rows: Vec<Vec<i32>> = get_rows_from_file(FILE_PATH)?;
    let mut safe_count: usize = 0;

    // Validate each row. valid => increment safe_count
    'row_loop: for row in rows {
        let row_length = row.len();

        // Skip row with 0 or 1 entries (not specified whether un/safe report)
        if row_length <= 1 {
            continue;
        }

        // Determine asc/desc trend based on first two values.
        let descending = match row[0].cmp(&row[1]) {
            Ordering::Equal => continue, // unsafe report ("adjacent levels differ by at least one")
            Ordering::Greater => true,
            Ordering::Less => false,
        };

        // Validate rules for row values.
        for i in 0..(row.len() - 1) {
            let val1 = row[i];
            let val2 = row[i + 1];

            // Validate asc/desc trend - rule #1 "The levels are either all increasing or all decreasing".
            match val1.cmp(&val2) {
                Ordering::Equal => continue 'row_loop, // unsafe report ("adjacent levels differ by at least one")
                Ordering::Greater => {
                    if !descending {
                        continue 'row_loop; // unsafe report
                    }
                }
                Ordering::Less => {
                    if descending {
                        continue 'row_loop; // unsafe report
                    }
                }
            }

            // Validate Δ - rule #2 "Any two adjacent levels differ by at least one and at most three".
            let values_delta = (val1 - val2).abs();

            if values_delta > 3 {
                continue 'row_loop; // unsafe report
            }
            // equality already checked in previous rule
        }

        safe_count += 1; // safe report
    }

    // Print result
    println!("Number of safe reports: {}", safe_count);

    Ok(())
}
