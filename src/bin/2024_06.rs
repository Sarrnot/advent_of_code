use std::{collections::HashSet, fs};

const FILE_PATH: &str = "./resources/2024_06.txt";

/// Solution for https://adventofcode.com/2024/day/6 - Part One & Two.
/// Run by `cargo run --bin 2024_06`.
/// Could be potentially optimized if each row/column had "cached" obstacle positions
/// and visiting grid points would be calculated not by iterating over each point individually => O(n)
/// but by creating "move tuples" - e.g. rows[4] has obstacles [7, 8] and visited points [(2, 6), (16, 9)] => O(1).
fn main() -> Result<(), String> {
    // Read file
    let file_content = match fs::read_to_string(FILE_PATH) {
        Ok(text) => text,
        Err(err) => return Err(err.to_string()),
    };

    // Parse file
    let grid = parse_file(&file_content);

    // Find guard
    let guard_coords = grid.find_guard_coords();
    let guard = Guard {
        direction: Direction::Up,
        x: guard_coords.0,
        y: guard_coords.1,
    };

    // Walk guard + recursive cycles (adding obstacles)
    let mut cyclic_counter = 0;
    let (is_cyclic, grid) = walk_guard(grid, guard, 0, 1, &mut cyclic_counter);
    if is_cyclic {
        panic!("First path is already cyclic.");
    };

    // Count visited
    let mut visited = 0;
    for row in grid.rows {
        for point in row {
            match point {
                GridPoint::Visited(_) => visited += 1,
                _ => (),
            }
        }
    }

    // Print result
    println!("Total number of visited positions: {}", visited);
    println!("Total number of possible cycles: {}", cyclic_counter);

    Ok(())
}

/// Returns whether the path is cyclic and the mutated Grid with visited points.
fn walk_guard(
    mut grid: Grid,
    mut guard: Guard,
    depth: usize,
    max_depth: usize,
    cyclic_counter: &mut usize,
) -> (bool, Grid) {
    // Walk until either out of bounds or cyclic.
    loop {
        // Visit current point (coords should be always valid here => unwrap())
        match grid.at(guard.coords()).unwrap() {
            GridPoint::Obstacle => {
                panic!("Data integrity violated, can't stand on an obstacle.");
            }
            GridPoint::Visited(prev_directions) => {
                let mut directions = prev_directions.clone();
                directions.insert(guard.direction);
                grid.set_point(GridPoint::Visited(directions), guard.coords());
            }
            _ => {
                let mut directions = HashSet::new();
                directions.insert(guard.direction);
                grid.set_point(GridPoint::Visited(directions), guard.coords());
            }
        };

        // Find next point, return if it would be out of bounds
        let next_coords = guard.next_coords();
        let next_point = match next_coords {
            Some(coords) => match grid.at(coords) {
                Some(point) => point,
                None => return (false, grid),
            },
            None => return (false, grid),
        };

        // Perform action
        match next_point {
            GridPoint::Obstacle => guard.turn(),
            GridPoint::Empty => {
                if depth < max_depth {
                    // Fork with an obstacle ahead
                    let mut grid_clone = grid.clone();
                    grid_clone.set_point(GridPoint::Obstacle, next_coords.unwrap());

                    let (is_cyclic, _) = walk_guard(
                        grid_clone,
                        guard.clone(),
                        depth + 1,
                        max_depth,
                        cyclic_counter,
                    );

                    if is_cyclic {
                        *cyclic_counter += 1;
                    }
                }

                guard.walk();
            }
            GridPoint::Visited(directions) => {
                // Return if cyclic, walk otherwise
                if directions.contains(&guard.direction) {
                    return (true, grid);
                }

                guard.walk();
            }
            GridPoint::Guard => panic!("Data integrity violated, guard found."),
        }
    }
}

fn parse_file(content: &str) -> Grid {
    let mut rows = vec![];

    for line in content.lines() {
        let mut row = vec![];

        for character in line.chars() {
            let grid_point = match character {
                '.' => GridPoint::Empty,
                '#' => GridPoint::Obstacle,
                '^' => GridPoint::Guard,
                _ => panic!("Invalid character."),
            };
            row.push(grid_point);
        }

        rows.push(row);
    }

    Grid { rows }
}

#[derive(Clone)]
enum GridPoint {
    Empty,
    Obstacle,
    Guard,
    Visited(HashSet<Direction>),
}

#[derive(Clone)]
struct Grid {
    rows: Vec<Vec<GridPoint>>,
}

impl Grid {
    fn at(&self, (x, y): (usize, usize)) -> Option<&GridPoint> {
        if y >= self.rows.len() || x >= self.rows[y].len() {
            return None;
        };

        Some(&self.rows[y][x])
    }

    fn set_point(&mut self, value: GridPoint, (x, y): (usize, usize)) {
        if y >= self.rows.len() || x >= self.rows[y].len() {
            panic!("Tried to set an out of bounds point.");
        };

        self.rows[y][x] = value;
    }

    fn find_guard_coords(&self) -> (usize, usize) {
        for (y, row) in (&self.rows).into_iter().enumerate() {
            for (x, point) in row.into_iter().enumerate() {
                match point {
                    GridPoint::Guard => return (x, y),
                    _ => (),
                }
            }
        }

        panic!("No guard found.");
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

#[derive(Clone)]
struct Guard {
    x: usize,
    y: usize,
    direction: Direction,
}

impl Guard {
    fn coords(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    fn turn(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn walk(&mut self) -> (usize, usize) {
        let (x, y) = self
            .next_coords()
            .expect("Tried to move to a negative index.");
        self.x = x;
        self.y = y;

        (x, y)
    }

    fn next_coords(&self) -> Option<(usize, usize)> {
        let new_coords = match self.direction {
            Direction::Up => {
                if self.y == 0 {
                    return None;
                } else {
                    (self.x, self.y - 1)
                }
            }
            Direction::Left => {
                if self.x == 0 {
                    return None;
                } else {
                    (self.x - 1, self.y)
                }
            }
            Direction::Right => (self.x + 1, self.y),
            Direction::Down => (self.x, self.y + 1),
        };

        Some(new_coords)
    }
}
