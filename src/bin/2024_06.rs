use std::fs;

const FILE_PATH: &str = "./resources/2024_06.txt";

/// Solution for https://adventofcode.com/2024/day/6.
/// Run by `cargo run --bin 2024_06`.
fn main() -> Result<(), String> {
    // Read file
    let file_content = match fs::read_to_string(FILE_PATH) {
        Ok(text) => text,
        Err(err) => return Err(err.to_string()),
    };

    // Parse file
    let mut grid = parse_file(&file_content);

    // Find guard
    let guard_coords = grid.find_guard_coords();
    let mut guard = Guard {
        direction: Direction::Up,
        x: guard_coords.0,
        y: guard_coords.1,
    };

    // Walk guard
    loop {
        // Visit current point
        grid.set_point(GridPoint::Visited, guard.coords());

        // Find next point, break if would be out of bounds
        let next_coords = guard.next_coords();
        let next_point = match next_coords {
            Some(coords) => match grid.at(coords) {
                Some(point) => point,
                None => break,
            },
            None => break,
        };

        // Perform action
        match next_point {
            GridPoint::Obstacle => guard.turn(),
            GridPoint::Empty | GridPoint::Visited => {
                guard.walk();
            }
            GridPoint::Guard => panic!("Data integrity violated, guard found."),
        }
    }

    // Count visited
    let mut visited = 0;
    for row in grid.rows {
        for point in row {
            match point {
                GridPoint::Visited => visited += 1,
                _ => (),
            }
        }
    }

    // Print result
    println!("Total number of visited positions: {}", visited);

    Ok(())
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

enum GridPoint {
    Empty,
    Obstacle,
    Guard,
    Visited,
}

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

enum Direction {
    Up,
    Left,
    Right,
    Down,
}

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
