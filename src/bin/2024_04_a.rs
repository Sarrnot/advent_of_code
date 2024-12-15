use std::fs;

const FILE_PATH: &str = "./resources/2024_04.txt";

/// Solution for https://adventofcode.com/2024/day/4 - Part One.
/// Run by `cargo run --bin 2024_04_a`.
fn main() -> Result<(), String> {
    // Read file
    let file_content = match fs::read_to_string(FILE_PATH) {
        Ok(text) => text,
        Err(err) => return Err(err.to_string()),
    };

    // Parse lines into grid
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in file_content.lines() {
        grid.push(line.chars().collect());
    }

    // Prepare directions
    let directions = vec![
        (AxisDirection::Negative, AxisDirection::Positive), // up left
        (AxisDirection::None, AxisDirection::Positive),     // up
        (AxisDirection::Positive, AxisDirection::Positive), // up right
        (AxisDirection::Negative, AxisDirection::None),     // left
        (AxisDirection::Positive, AxisDirection::None),     // right
        (AxisDirection::Negative, AxisDirection::Negative), // down left
        (AxisDirection::None, AxisDirection::Negative),     // down
        (AxisDirection::Positive, AxisDirection::Negative), // down right
    ];

    // Find each 'X' and try each direction from it.
    let mut total_count = 0;

    for (y, row) in (&grid).into_iter().enumerate() {
        for (x, char) in row.into_iter().enumerate() {
            if *char != 'X' {
                continue;
            }

            for direction in &directions {
                if test_direction(&grid, x, y, direction) {
                    total_count += 1;
                };
            }
        }
    }

    // Print result
    println!("Total count of XMAS: {}", total_count);

    Ok(())
}

fn test_direction(
    grid: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    direction: &(AxisDirection, AxisDirection),
) -> bool {
    for (i, character) in vec!['M', 'A', 'S'].into_iter().enumerate() {
        // overflows not handled for simplicity
        let distance = i + 1;

        let tested_x = match direction.0 {
            AxisDirection::Positive => x + distance,
            AxisDirection::Negative => match x.checked_sub(distance) {
                Some(val) => val,
                None => return false, // x out of bounds (less than 0)
            },
            AxisDirection::None => x,
        };
        let tested_y = match direction.1 {
            AxisDirection::Positive => y + distance,
            AxisDirection::Negative => match y.checked_sub(distance) {
                Some(val) => val,
                None => return false, // y out of bounds (less than 0)
            },
            AxisDirection::None => y,
        };

        let out_of_bounds = tested_y >= grid.len() || tested_x >= grid[tested_y].len(); // less than 0 already checked

        if out_of_bounds {
            return false;
        }

        if grid[tested_y][tested_x] != character {
            return false;
        }
    }

    true
}

enum AxisDirection {
    Positive,
    Negative,
    None,
}
