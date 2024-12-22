use std::{collections::HashSet, fs};

const FILE_PATH: &str = "./resources/2024_10.txt";

/// Solution for https://adventofcode.com/2024/day/10 - Part One & Two.
/// Run by `cargo run --bin 2024_10`.
fn main() {
    // Read and parse file
    let file_content = fs::read_to_string(FILE_PATH).unwrap();
    let grid = parse_file(&file_content);

    // Calculate
    let mut score_sum = 0;
    let mut rating_sum = 0;

    for (y, row) in (&grid.rows).into_iter().enumerate() {
        for (x, height) in row.into_iter().enumerate() {
            if *height != 0 {
                continue;
            }

            let mut mountain_peaks = HashSet::new();
            test_position(&grid, (x, y), 0, 9, &mut mountain_peaks, &mut rating_sum);
            score_sum += mountain_peaks.len();
        }
    }

    // Print result
    println!("Score sum: {}", score_sum);
    println!("Rating sum: {}", rating_sum);
}

fn test_position(
    grid: &Grid,
    (x, y): Coordinates,
    height: u8,
    max_height: u8,
    mountain_peaks: &mut HashSet<Coordinates>,
    rating_sum: &mut usize,
) {
    // Reached top => increase score
    if height == max_height {
        mountain_peaks.insert((x, y));
        *rating_sum += 1;
        return;
    }

    // Try each direction (whether an increment of current height)
    let mut try_direction = |coords: Coordinates| match grid.at(coords) {
        Some(new_height) => {
            if new_height == height + 1 {
                test_position(
                    grid,
                    coords,
                    height + 1,
                    max_height,
                    mountain_peaks,
                    rating_sum,
                );
            }
        }
        None => (),
    };

    // Left
    if x > 0 {
        try_direction((x - 1, y));
    }

    // Right
    try_direction((x + 1, y));

    // Up
    if y > 0 {
        try_direction((x, y - 1));
    }

    // Down
    try_direction((x, y + 1));
}

fn parse_file(content: &str) -> Grid {
    let mut rows = vec![];

    for line in content.lines() {
        rows.push(
            line.chars()
                .map(|val| val.to_digit(10).unwrap().try_into().unwrap())
                .collect(),
        );
    }

    Grid { rows }
}

type Coordinates = (usize, usize);
struct Grid {
    rows: Vec<Vec<u8>>,
}

impl Grid {
    fn at(&self, (x, y): Coordinates) -> Option<u8> {
        if y >= self.rows.len() || x >= self.rows[y].len() {
            return None;
        }

        Some(self.rows[y][x])
    }
}
