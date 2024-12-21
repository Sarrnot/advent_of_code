use std::{collections::HashMap, fs, isize};

const FILE_PATH: &str = "./resources/2024_08.txt";

/// Solution for https://adventofcode.com/2024/day/8 - Part One & Two.
/// Run by `cargo run --bin 2024_08`.
fn main() {
    // Read file and parse file
    let file_content = fs::read_to_string(FILE_PATH).unwrap();
    let (nodes_map, antinode_grid) = parse_file(&file_content);

    // Calculate results
    let non_repeated_count = calculate_antinode_count(&nodes_map, antinode_grid.clone(), false);
    let repeated_count = calculate_antinode_count(&nodes_map, antinode_grid.clone(), true);

    // Print result
    println!("Antinode position count: {}", non_repeated_count);
    println!("Repeated antinode position count: {}", repeated_count);
}

fn calculate_antinode_count(
    nodes_map: &NodesMap,
    mut antinode_grid: AntinodeGrid,
    repeat: bool,
) -> usize {
    // For each node frequency
    for (_, coords_list) in nodes_map {
        // Test all combinations of nodes
        for (i, coords1) in coords_list.into_iter().enumerate() {
            for coords2 in &coords_list[i + 1..coords_list.len()] {
                let (dist_x, dist_y) = coords1.distance(coords2);

                let mut create_antinodes = |coords: &mut Coordinates, distance: Distance| loop {
                    match coords.safe_add_distance(distance) {
                        Some(val) => {
                            let out_of_bounds =
                                val.y >= antinode_grid.len() || val.x >= antinode_grid[val.y].len();
                            if out_of_bounds {
                                break;
                            }
                            antinode_grid[val.y][val.x] = true;
                        }
                        None => break,
                    };

                    if !repeat {
                        break;
                    }
                };

                match repeat {
                    true => {
                        create_antinodes(&mut coords2.clone(), (-dist_x, -dist_y));
                        create_antinodes(&mut coords1.clone(), (dist_x, dist_y));
                    }
                    false => {
                        create_antinodes(&mut coords1.clone(), (-dist_x, -dist_y));
                        create_antinodes(&mut coords2.clone(), (dist_x, dist_y));
                    }
                }
            }
        }
    }

    let mut count = 0;
    for row in antinode_grid {
        for point in row {
            if point {
                count += 1;
            }
        }
    }

    count
}

fn parse_file(content: &str) -> (NodesMap, AntinodeGrid) {
    let mut nodes_map: NodesMap = HashMap::new();
    let mut antinode_grid: AntinodeGrid = vec![];

    for (y, line) in content.lines().enumerate() {
        let chars: Vec<char> = line.chars().collect();

        antinode_grid.push(vec![false; chars.len()]);

        for (x, char) in chars.into_iter().enumerate() {
            if char == '.' {
                continue;
            }

            match nodes_map.get_mut(&char) {
                Some(coords_list) => coords_list.push(Coordinates { x, y }),
                None => {
                    nodes_map.insert(char, vec![Coordinates { x, y }]);
                }
            }
        }
    }

    (nodes_map, antinode_grid)
}

type AntinodeGrid = Vec<Vec<bool>>;
type NodesMap = HashMap<char, Vec<Coordinates>>;
type Distance = (isize, isize);

#[derive(Clone)]
struct Coordinates {
    x: usize,
    y: usize,
}

impl Coordinates {
    fn distance(&self, coords: &Coordinates) -> Distance {
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

    fn safe_add_distance(&mut self, distance: Distance) -> Option<&mut Self> {
        let new_x = self.x_isize() + distance.0;
        let new_y = self.y_isize() + distance.1;

        if new_x < 0 || new_y < 0 {
            return None;
        }

        self.x = new_x as usize;
        self.y = new_y as usize;

        Some(self)
    }
}
