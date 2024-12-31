use std::{cmp::Ordering, collections::HashMap, fs};

use regex::Regex;

const FILE_PATH: &str = "./resources/2024_14.txt";

const X_LEN: isize = 101;
const Y_LEN: isize = 103;
const ITER_COUNT: isize = 100;

// TODO: Part Two
/// Solution for https://adventofcode.com/2024/day/14 - Part One.
/// Run by `cargo run --bin 2024_14`.
fn main() {
    if X_LEN % 2 == 0 || Y_LEN % 2 == 0 {
        panic!("find_quadrant() does not take into account even number of positions.");
    }

    // Read and parse file
    let file_content = fs::read_to_string(FILE_PATH).unwrap();
    let robots = parse_file(&file_content);

    // Find robot count in quadrants after ITER_COUNT
    let mut quadrant_counts: HashMap<usize, usize> =
        HashMap::from([(0, 0), (1, 0), (2, 0), (3, 0)]);

    for robot in &robots {
        let new_x_signed = (robot.position.0 + robot.velocity.0 * ITER_COUNT) % X_LEN;
        let new_y_signed = (robot.position.1 + robot.velocity.1 * ITER_COUNT) % Y_LEN;

        let new_x = if new_x_signed >= 0 {
            new_x_signed
        } else {
            X_LEN + new_x_signed
        };

        let new_y = if new_y_signed >= 0 {
            new_y_signed
        } else {
            Y_LEN + new_y_signed
        };

        let quadrant = match find_quadrant((new_x, new_y)) {
            Some(val) => val,
            None => continue,
        };

        let quadrant_count = quadrant_counts.get_mut(&quadrant).unwrap();
        *quadrant_count += 1;
    }

    // Calculate and print safety factor
    let safety_factor = quadrant_counts
        .into_iter()
        .fold(1, |acc, (_, count)| acc * count);

    println!("Safety factor: {}", safety_factor);
}

fn parse_file(content: &str) -> Vec<Robot> {
    let mut robots = vec![];

    let number_regex = Regex::new(r"-?[0-9]+").unwrap();

    for line in content.lines() {
        let numbers: Vec<isize> = number_regex
            .find_iter(line)
            .map(|val| val.as_str().parse().unwrap())
            .collect();

        let position = (numbers[0], numbers[1]);
        let velocity = (numbers[2], numbers[3]);

        robots.push(Robot { position, velocity });
    }

    robots
}

fn find_quadrant((x, y): (isize, isize)) -> Option<usize> {
    match (x.cmp(&((X_LEN - 1) / 2)), y.cmp(&((Y_LEN - 1) / 2))) {
        (Ordering::Equal, _) => None,
        (_, Ordering::Equal) => None,
        (Ordering::Less, Ordering::Less) => Some(0),
        (Ordering::Greater, Ordering::Less) => Some(1),
        (Ordering::Less, Ordering::Greater) => Some(2),
        (Ordering::Greater, Ordering::Greater) => Some(3),
    }
}

struct Robot {
    position: (isize, isize),
    velocity: (isize, isize),
}
