use std::cell::RefCell;
use std::fs;
use std::rc::Rc;
use advent_of_code::{Coordinates, Direction, Distance, Grid};

const FILE_PATH: &str = "./resources/2024_15.txt";

/// Solution for https://adventofcode.com/2024/day/15 - Part One & Two.
/// Run by `cargo run --bin 2024_15`.
/// This solution could be heavily optimized and made more generic, it is essentially a simple game/physics engine.
fn main() {
    // Read and parse file
    let file_content = fs::read_to_string(FILE_PATH).unwrap();
    let (mut grid, movement_instructions, robot_coords) = parse_file(&file_content);
    let mut enlarged_grid = create_enlarged_grid(&grid);
    let robot_coords_enlarged = Coordinates::new((robot_coords.x * 2, robot_coords.y));

    apply_movement_instructions(&movement_instructions, &mut grid, robot_coords.clone());
    apply_movement_instructions(&movement_instructions, &mut enlarged_grid, robot_coords_enlarged.clone());

    let result = calculate_result(&grid);
    let result_enlarged = calculate_result(&enlarged_grid);

    println!("Result: {}", result);
    println!("Result - enlarged grid: {}", result_enlarged);
}

fn apply_movement_instructions(movement_instructions: &MovementInstructions, grid: &mut PointGrid, mut robot_coords: Coordinates) {
    for instruction in movement_instructions {
        let step_distance = instruction.step_distance();
        let new_coords = robot_coords.clone().safe_add_distance(step_distance).expect("Can't be out of bounds - wall around grid.");
        let target_point = grid.at(&new_coords).expect("Can't be out of bounds - wall around grid.");

        match target_point {
            // Move to empty space
            Point::Empty => (),
            // Obstacle => do nothing
            Point::Obstacle => continue,
            // Try to move box(es)
            Point::Box(box_ref) => {
                // Find boxes and check if they can be moved
                let mut boxes_to_move = vec![];
                if !can_move_box_recursive(box_ref, instruction, grid, &mut boxes_to_move) {
                    continue;
                }

                // Move boxes
                for box_to_move in boxes_to_move {
                    for coords in box_to_move.borrow().get_all_coords() {
                        let old_box_point = grid.at_mut(&coords).expect("Should always exist");
                        *old_box_point = Point::Empty;
                    }

                    box_to_move.borrow_mut().move_by(step_distance).expect("Should always be able to move");

                    for coords in box_to_move.borrow().get_all_coords() {
                        let new_box_point = grid.at_mut(&coords).expect("Should always exist");
                        *new_box_point = Point::Box(Rc::clone(&box_to_move));
                    }
                }
            }
            Point::Robot => panic!("Does not support multiple robots"),
        }

        // Move robot
        *grid.at_mut(&robot_coords).unwrap() = Point::Empty;
        robot_coords = new_coords;
        *grid.at_mut(&robot_coords).unwrap() = Point::Robot;
    }
}

/// This function is quite inefficient (duplicate searches, edge_coords could be done with an iterator, etc.)
/// and could be heavily optimized.
fn can_move_box_recursive(target_box: &BoxRef, instruction: &Direction, grid: &PointGrid, boxes_to_move: &mut Vec<BoxRef>) -> bool {
    let step_distance = instruction.step_distance();
    let box_edge = target_box.borrow().get_edge_coords(instruction);

    for edge_coords in &box_edge {
        let lookup_coords = edge_coords.clone().safe_add_distance(step_distance).expect("Can't be out of bounds - wall around grid.");
        match grid.at(&lookup_coords).expect("Can't be out of bounds - wall around grid.") {
            Point::Empty => (),
            Point::Obstacle => return false,
            Point::Box(next_box) => {
                if !can_move_box_recursive(next_box, instruction, grid, boxes_to_move) {
                    return false;
                }
            }
            Point::Robot => panic!("Does not support multiple robots"),
        }
    }

    if !boxes_to_move.contains(target_box) {
        boxes_to_move.push(Rc::clone(target_box)); // Temporarily "move" ownership out of grid while boxes are being moved
    }

    true
}

fn calculate_result(grid: &PointGrid) -> usize {
    let mut result = 0;

    for (y, row) in grid.get_rows().iter().enumerate() {
        for (x, point) in row.iter().enumerate() {
            match point {
                Point::Box(box_ref) => {
                    if box_ref.borrow().origin != Coordinates::new((x,y)) {
                        continue;
                    }
                },
                _ => continue,
            }

            result += 100 * y + x;
        }
    }

    result
}

/// returns tuple of (grid, movement instructions, robot starting position)
fn parse_file(content: &str) -> (PointGrid, MovementInstructions, Coordinates) {
    let mut rows = vec![];
    let mut move_instructions = vec![];
    let mut coords = Coordinates::new((0, 0));

    let mut line_iterator = content.lines();

    for (y, line) in line_iterator.by_ref().enumerate() {
        if line.is_empty() {
            break;
        }

        let mut row = vec![];

        for (x, char) in line.chars().enumerate() {
            let point = match char {
                '#' => Point::Obstacle,
                'O' => {
                    let box_object = BoxObject {
                        origin: Coordinates::new((x, y)),
                        dimensions: (1, 1)
                    };
                    Point::Box(Rc::new(RefCell::new(box_object)))
                },
                '.' => Point::Empty,
                '@' => {
                    coords.x = x;
                    coords.y = y;
                    Point::Robot
                }
                _ => panic!("Invalid grid character."),
            };
            row.push(point);
        }

        rows.push(row);
    }

    for line in line_iterator.by_ref() {
        for char in line.chars() {
            let move_instruction = match char {
                '^' => Direction::Up,
                'v' => Direction::Down,
                '>' => Direction::Right,
                '<' => Direction::Left,
                _ => panic!("Invalid move character."),
            };
            move_instructions.push(move_instruction);
        }
    }

    (Grid::new(rows), move_instructions, coords)
}

fn create_enlarged_grid(grid: &PointGrid) -> PointGrid {
    let mut new_grid = Grid::new(vec![]);

    for row in grid.get_rows().iter() {
        let mut new_row = vec![];

        for point in row.iter() {
            match point {
                Point::Empty => {
                    new_row.push(Point::Empty);
                    new_row.push(Point::Empty);
                }
                Point::Obstacle => {
                    new_row.push(Point::Obstacle);
                    new_row.push(Point::Obstacle);
                }
                Point::Box(box_object) => {
                    let mut cloned_box_object = box_object.borrow().clone();
                    cloned_box_object.origin.x *= 2;
                    cloned_box_object.dimensions.0 *= 2;

                    let rc = Rc::new(RefCell::new(cloned_box_object));
                    new_row.push(Point::Box(Rc::clone(&rc)));
                    new_row.push(Point::Box(Rc::clone(&rc)));
                }
                Point::Robot => {
                    new_row.push(Point::Robot);
                    new_row.push(Point::Empty);
                }
            }
        }

        new_grid.get_rows_mut().push(new_row);
    };

    new_grid
}

fn _print_grid(grid: &PointGrid) {
    println!();
    for row in grid.get_rows().iter() {
        for point in row.iter() {
            match point {
                Point::Empty => print!("."),
                Point::Obstacle => print!("#"),
                Point::Box(_) => print!("O"),
                Point::Robot => print!("@"),
            }
        }
        println!();
    }
    println!();
}

type PointGrid = Grid<Point>;
type MovementInstructions = Vec<Direction>;
type BoxRef = Rc<RefCell<BoxObject>>;

#[derive(PartialEq, Clone, Debug)]
struct BoxObject {
    /// Top left corner
    origin: Coordinates,
    dimensions: (usize, usize),
}

impl BoxObject {
    fn move_by(&mut self, distance: Distance) -> Result<(), ()> {
        match self.origin.clone().safe_add_distance(distance) {
            Some(new_origin) => {
                self.origin = new_origin;
                Ok(())
            }
            None => Err(())
        }
    }

    fn get_edge_coords(&self, direction: &Direction) -> Vec<Coordinates> {
        let mut coords = vec![];

        let variable_coord = match direction {
            Direction::Up|Direction::Down => self.origin.x..(self.origin.x + self.dimensions.0),
            Direction::Left|Direction::Right => self.origin.y..(self.origin.y + self.dimensions.1),
        };

        let constant_coord = match direction {
            Direction::Up => self.origin.y,
            Direction::Down => self.origin.y + self.dimensions.1 - 1,
            Direction::Left => self.origin.x,
            Direction::Right => self.origin.x + self.dimensions.0 - 1,
        };

        match direction {
            Direction::Up|Direction::Down => {
                for x in variable_coord {
                    coords.push(Coordinates::new((x, constant_coord)));
                }
            }
            Direction::Left|Direction::Right => {
                for y in variable_coord {
                    coords.push(Coordinates::new((constant_coord, y)));
                }
            }
        }

        coords
    }

    fn get_all_coords(&self) -> Vec<Coordinates> {
        let mut coords = vec![];

        for x in self.origin.x..(self.origin.x + self.dimensions.0) {
            for y in self.origin.y..(self.origin.y + self.dimensions.1) {
                coords.push(Coordinates::new((x, y)));
            }
        }

        coords
    }
}

#[derive(PartialEq)]
enum Point {
    Obstacle,
    Box(BoxRef),
    Empty,
    Robot
}
