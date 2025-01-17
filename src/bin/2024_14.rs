use std::{cmp::Ordering, collections::HashMap, fs};

use advent_of_code::{Coordinates, Distance, Grid};
use regex::Regex;

const FILE_PATH: &str = "./resources/2024_14.txt";

const X_LEN: usize = 101;
const Y_LEN: usize = 103;
const ITER_COUNT: usize = 100;
const DIRECTIONS: [Distance; 8] = [
    (-1, 1),  // up left
    (0, 1),   // up middle
    (1, 1),   // up right
    (-1, 0),  // left
    (1, 0),   // right
    (-1, -1), // down left
    (0, -1),  // down middle
    (1, -1),  // down right
];
const HEURISTIC_THRESHOLD: usize = 500;

/// Solution for https://adventofcode.com/2024/day/14 - Part One & Two.
/// Run by `cargo run --bin 2024_14`.
fn main() {
    if X_LEN % 2 == 0 || Y_LEN % 2 == 0 {
        panic!("find_quadrant() does not take into account even number of positions.");
    }

    // Read and parse file
    let file_content = fs::read_to_string(FILE_PATH).unwrap();
    let mut robots = parse_file(&file_content);

    // Find robot count in quadrants after ITER_COUNT
    let mut quadrant_counts: HashMap<usize, usize> =
        HashMap::from([(0, 0), (1, 0), (2, 0), (3, 0)]);

    for robot in &robots {
        let new_coords = get_new_position(robot, ITER_COUNT);

        let quadrant = match find_quadrant(new_coords) {
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

    // Find Easter egg
    let mut counter = 1;
    loop {
        // Move robots by one iteration
        for robot in &mut robots {
            robot.position = get_new_position(robot, 1);
        }

        // Create robots grid
        let mut grid = Grid::new(vec![vec![GridPoint::Empty; X_LEN]; Y_LEN]);
        mark_grid_positions(&robots, &mut grid);

        // Search for image
        // Heuristic - anything that resembles some kind of image will very likely have many neighbouring points forming continuous lines or shapes.
        if get_continuity_score(&grid, &robots) > HEURISTIC_THRESHOLD {
            println!();
            print_grid(&grid);
            println!();
            break;
        };

        counter += 1;
    }

    println!("Easter egg after: {} iterations", counter);
}

/// Get coordinates after specified number of iterations. O(1) complexity.
fn get_new_position(robot: &Robot, iter_count: usize) -> Coordinates {
    let new_x_signed =
        (robot.position.x as isize + robot.velocity.0 * iter_count as isize) % X_LEN as isize;
    let new_y_signed =
        (robot.position.y as isize + robot.velocity.1 * iter_count as isize) % Y_LEN as isize;

    let new_x = if new_x_signed >= 0 {
        new_x_signed
    } else {
        X_LEN as isize + new_x_signed
    };

    let new_y = if new_y_signed >= 0 {
        new_y_signed
    } else {
        Y_LEN as isize + new_y_signed
    };

    Coordinates::new((new_x as usize, new_y as usize))
}

/// Place robots on an empty grid.
fn mark_grid_positions(robots: &Vec<Robot>, grid: &mut RobotGrid) {
    for robot in robots {
        *grid.at_mut(&robot.position).unwrap() = GridPoint::Robot;
    }
}

/// Get score based on neighbouring points (each neighbour increments score by 1).
fn get_continuity_score(grid: &RobotGrid, robots: &Vec<Robot>) -> usize {
    let mut score = 0;

    for robot in robots {
        for direction in DIRECTIONS {
            if let Some(coords) = robot.position.clone().safe_add_distance(direction) {
                if let Some(point) = grid.at(&coords) {
                    if *point == GridPoint::Robot {
                        score += 1;
                    }
                }
            }
        }
    }

    score
}

fn print_grid(grid: &RobotGrid) {
    for row in grid.get_rows() {
        println!();
        for point in row {
            match point {
                GridPoint::Robot => print!("X"),
                GridPoint::Empty => print!(" "),
            }
        }
    }

    println!();
}

fn parse_file(content: &str) -> Vec<Robot> {
    let mut robots = vec![];

    let number_regex = Regex::new(r"-?[0-9]+").unwrap();

    for line in content.lines() {
        let numbers: Vec<isize> = number_regex
            .find_iter(line)
            .map(|val| val.as_str().parse().unwrap())
            .collect();

        let position = Coordinates::new((
            numbers[0].try_into().unwrap(),
            numbers[1].try_into().unwrap(),
        ));
        let velocity = (numbers[2], numbers[3]);

        robots.push(Robot { position, velocity });
    }

    robots
}

fn find_quadrant(coords: Coordinates) -> Option<usize> {
    match (
        coords.x.cmp(&((X_LEN - 1) / 2)),
        coords.y.cmp(&((Y_LEN - 1) / 2)),
    ) {
        (Ordering::Equal, _) => None,
        (_, Ordering::Equal) => None,
        (Ordering::Less, Ordering::Less) => Some(0),
        (Ordering::Greater, Ordering::Less) => Some(1),
        (Ordering::Less, Ordering::Greater) => Some(2),
        (Ordering::Greater, Ordering::Greater) => Some(3),
    }
}

struct Robot {
    position: Coordinates,
    velocity: Distance,
}

#[derive(Clone, PartialEq)]
enum GridPoint {
    Robot,
    Empty,
}

type RobotGrid = Grid<GridPoint>;
