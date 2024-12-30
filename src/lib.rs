use std::{fs, str::FromStr};

pub fn get_columns_from_file<T: FromStr, const N: usize>(
    file_path: &str,
) -> Result<[Vec<T>; N], String> {
    // Read file
    let file_content = match fs::read_to_string(file_path) {
        Ok(text) => text,
        Err(err) => return Err(err.to_string()),
    };

    // Create empty columns
    let mut columns = [const { Vec::new() }; N];

    // Parse each line
    for (i, line) in file_content.lines().enumerate() {
        // Split values by whitespace
        let parts = line.split_whitespace();
        let line_num = i + 1;

        // Validate - has N values separated by whitespace
        if parts.clone().count() != N {
            return Err(format!(
                "Error on line {line_num}. Must have {N} values separated by whitespace."
            ));
        }

        // Parse and save all values on current line.
        for (j, raw_value) in parts.enumerate() {
            match raw_value.parse::<T>() {
                Ok(value) => {
                    columns[j].push(value);
                }
                Err(_) => {
                    return Err(format!("Invalid value '{raw_value}' on line {line_num}."));
                }
            }
        }
    }

    Ok(columns)
}

pub fn get_rows_from_file<T: FromStr>(file_path: &str) -> Result<Vec<Vec<T>>, String> {
    // Read file
    let file_content = match fs::read_to_string(file_path) {
        Ok(text) => text,
        Err(err) => return Err(err.to_string()),
    };

    // Create rows
    let mut rows = Vec::new();

    // Parse each line
    for (i, line) in file_content.lines().enumerate() {
        let mut row = Vec::new();

        // Split values by whitespace
        let parts = line.split_whitespace();
        let line_num = i + 1;

        // Parse and save all values on current line.
        for raw_value in parts {
            match raw_value.parse::<T>() {
                Ok(value) => {
                    row.push(value);
                }
                Err(_) => {
                    return Err(format!("Invalid value '{raw_value}' on line {line_num}."));
                }
            }
        }

        // Add row
        rows.push(row);
    }

    Ok(rows)
}

pub struct Grid<T> {
    rows: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn new(rows: Vec<Vec<T>>) -> Self {
        Self { rows }
    }

    pub fn at(&self, coords: &Coordinates) -> Option<&T> {
        if !self.are_coords_in_bounds(coords) {
            return None;
        }

        Some(&self.rows[coords.y][coords.x])
    }

    pub fn at_mut(&mut self, coords: &Coordinates) -> Option<&mut T> {
        if !self.are_coords_in_bounds(coords) {
            return None;
        }

        Some(&mut self.rows[coords.y][coords.x])
    }

    pub fn get_row(&self, y: usize) -> Option<&Vec<T>> {
        if !self.is_y_in_bounds(y) {
            return None;
        }

        Some(&self.rows[y])
    }

    pub fn get_rows(&self) -> &Vec<Vec<T>> {
        &self.rows
    }

    fn are_coords_in_bounds(&self, coords: &Coordinates) -> bool {
        self.is_y_in_bounds(coords.y) && coords.x < self.rows[coords.y].len()
    }

    fn is_y_in_bounds(&self, y: usize) -> bool {
        y < self.rows.len()
    }
}

pub type Distance = (isize, isize);

#[derive(Clone)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize,
}

impl Coordinates {
    pub fn new((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }

    pub fn safe_add_distance(mut self, distance: Distance) -> Option<Self> {
        let new_x = self.x_isize() + distance.0;
        let new_y = self.y_isize() + distance.1;

        if new_x < 0 || new_y < 0 {
            return None;
        }

        self.x = new_x as usize;
        self.y = new_y as usize;

        Some(self)
    }

    pub fn distance(&self, coords: &Coordinates) -> Distance {
        (
            coords.x_isize() - self.x_isize(),
            coords.y_isize() - self.y_isize(),
        )
    }

    fn x_isize(&self) -> isize {
        self.x.try_into().unwrap()
    }

    fn y_isize(&self) -> isize {
        self.y.try_into().unwrap()
    }
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn step_distance(&self) -> Distance {
        match self {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
        }
    }

    pub fn is_horizontal(&self) -> bool {
        match self {
            Direction::Left | Direction::Right => true,
            Direction::Up | Direction::Down => false,
        }
    }
}

pub const DIRECTIONS: [Direction; 4] = [
    Direction::Left,
    Direction::Right,
    Direction::Up,
    Direction::Down,
];
