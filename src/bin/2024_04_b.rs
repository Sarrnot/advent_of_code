use std::fs;

const FILE_PATH: &str = "./resources/2024_04.txt";

/// Solution for https://adventofcode.com/2024/day/4 - Part Two.
/// Run by `cargo run --bin 2024_04_b`.
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

    // Find each 'A' and test it.
    let mut total_count = 0;

    for (y, row) in (&grid).into_iter().enumerate() {
        for (x, char) in row.into_iter().enumerate() {
            if *char != 'A' {
                continue;
            }

            if test_cross(&grid, x, y) {
                total_count += 1;
            }
        }
    }

    // Print result
    println!("Total count of X-MAS: {}", total_count);

    Ok(())
}

fn test_cross(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let is_on_edge = y == 0 || x == 0 || y + 1 >= grid.len() || x + 1 >= grid[y + 1].len();

    if is_on_edge {
        return false;
    }

    let left_up = grid[y - 1][x - 1];
    let right_up = grid[y - 1][x + 1];
    let left_down = grid[y + 1][x - 1];
    let right_down = grid[y + 1][x + 1];

    match (left_up, right_down) {
        ('M', 'S') => true,
        ('S', 'M') => true,
        _ => return false,
    };

    match (right_up, left_down) {
        ('M', 'S') => true,
        ('S', 'M') => true,
        _ => return false,
    };

    true
}
