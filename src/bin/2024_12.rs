use std::{collections::HashMap, fs};

use advent_of_code::{Coordinates, Direction, Grid, DIRECTIONS};

const FILE_PATH: &str = "./resources/2024_12.txt";

/// Solution for https://adventofcode.com/2024/day/12 - Part One & Two.
/// Run by `cargo run --bin 2024_12`.
fn main() {
    // Read and parse file
    let file_content = fs::read_to_string(FILE_PATH).unwrap();
    let garden_plot_grid = parse_file(&file_content);

    // Find regions
    let (regions_grid, highest_region_id) = create_regions_grid(&garden_plot_grid);

    // Create areas, perimeters and sides maps [region_id => count].
    let areas_map = get_areas_map(&regions_grid);
    let perimeters_map = get_perimeters_map(&regions_grid);
    let sides_map = get_sides_map(&regions_grid);

    // Calculate price
    let mut price_perimeter = 0;
    let mut price_sides = 0;
    for id in 1..highest_region_id {
        let area = areas_map.get(&id).unwrap();
        price_perimeter += area * perimeters_map.get(&id).unwrap();
        price_sides += area * sides_map.get(&id).unwrap();
    }

    // Print result
    println!("Total fence price - perimeter: {}", price_perimeter);
    println!("Total fence price - sides: {}", price_sides);
}

fn parse_file(content: &str) -> GardenPlotGrid {
    let mut garden_plot_grid: Vec<Vec<char>> = vec![];

    for line in content.lines() {
        garden_plot_grid.push(line.chars().collect());
    }

    GardenPlotGrid::new(garden_plot_grid)
}

fn create_regions_grid(plot_grid: &GardenPlotGrid) -> (RegionGrid, usize) {
    // Create empty regions grid - id 0 represents "no id"
    let mut region_grid: RegionGrid =
        RegionGrid::new(vec![
            vec![0; plot_grid.get_row(0).unwrap().len()];
            plot_grid.get_rows().len()
        ]);

    let mut region_id_counter = 1;

    // Fill grid with ids
    for (y, row) in plot_grid.get_rows().into_iter().enumerate() {
        for (x, plot_id) in row.into_iter().enumerate() {
            let coords = Coordinates::new((x, y));
            let region_id = region_grid.at(&coords).unwrap();

            // Id already found => skip
            if *region_id != 0 {
                continue;
            }

            // Recursively search from current point
            mark_region_points(
                coords,
                *plot_id,
                region_id_counter,
                &mut region_grid,
                &plot_grid,
            );

            region_id_counter += 1;
        }
    }

    (region_grid, region_id_counter)
}

/// Recursively search for all region points and save them to grid
fn mark_region_points(
    coords: Coordinates,
    plot_id: char,
    region_id: usize,
    regions_grid: &mut RegionGrid,
    garden_plot_grid: &GardenPlotGrid,
) {
    // Save current point to grid
    *regions_grid.at_mut(&coords).unwrap() = region_id;

    // Try each direction
    for direction in DIRECTIONS {
        // Would be out of bounds => continue
        let target_coords = match coords.clone().safe_add_distance(direction.step_distance()) {
            Some(val) => val,
            None => continue,
        };

        // Other garden plot id => continue
        match garden_plot_grid.at(&target_coords) {
            Some(target_plot_id) => {
                if *target_plot_id != plot_id {
                    continue;
                }
            }
            None => continue,
        }

        // Already searched => continue
        match regions_grid.at(&target_coords) {
            Some(target_region_id) => {
                if *target_region_id == region_id {
                    continue;
                }
            }
            None => continue,
        }

        // Recurse to neighbour point
        mark_region_points(
            target_coords,
            plot_id,
            region_id,
            regions_grid,
            garden_plot_grid,
        );
    }
}

fn get_areas_map(regions_grid: &RegionGrid) -> HashMap<usize, usize> {
    let mut areas_map = HashMap::new();

    for row in regions_grid.get_rows() {
        for region_id in row {
            let area = areas_map.entry(*region_id).or_insert(0);

            *area += 1;
        }
    }

    areas_map
}

fn get_perimeters_map(regions_grid: &RegionGrid) -> HashMap<usize, usize> {
    let mut perimeters_map = HashMap::new();

    for (y, row) in regions_grid.get_rows().into_iter().enumerate() {
        for (x, region_id) in row.into_iter().enumerate() {
            let perimeter = perimeters_map.entry(*region_id).or_insert(0);

            let coords = Coordinates::new((x, y));

            for direction in DIRECTIONS {
                if !is_edge(&coords, &direction, region_id, regions_grid) {
                    continue;
                };

                *perimeter += 1;
            }
        }
    }

    perimeters_map
}

fn get_sides_map(regions_grid: &RegionGrid) -> HashMap<usize, usize> {
    let mut sides_map = HashMap::new();

    // For each point
    for (y, row) in regions_grid.get_rows().into_iter().enumerate() {
        for (x, region_id) in row.into_iter().enumerate() {
            let sides = sides_map.entry(*region_id).or_insert(0);

            // Try each direction - if first point on a side => increment
            for direction in DIRECTIONS {
                let coords = Coordinates::new((x, y));

                // Current point is not edge => continue
                if !is_edge(&coords, &direction, region_id, regions_grid) {
                    continue;
                };

                // Is not first side point => continue
                let prev_direction = match direction.is_horizontal() {
                    true => Direction::Up,
                    false => Direction::Left,
                };
                if let Some(prev_coords) = coords
                    .clone()
                    .safe_add_distance(prev_direction.step_distance())
                {
                    if let Some(prev_region_id) = regions_grid.at(&prev_coords) {
                        if prev_region_id == region_id
                            && is_edge(&prev_coords, &direction, region_id, regions_grid)
                        {
                            continue;
                        }
                    }
                }

                // Increment side count
                *sides += 1;
            }
        }
    }

    sides_map
}

fn is_edge(
    coords: &Coordinates,
    direction: &Direction,
    region_id: &usize,
    regions_grid: &RegionGrid,
) -> bool {
    if let Some(neighbour_coords) = coords.clone().safe_add_distance(direction.step_distance()) {
        if let Some(neighbour_region_id) = regions_grid.at(&neighbour_coords) {
            if neighbour_region_id == region_id {
                return false;
            }
        }
    }

    true
}

type GardenPlotGrid = Grid<char>;
type RegionGrid = Grid<usize>;
