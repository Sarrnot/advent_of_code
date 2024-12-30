use conv::*;
use std::fs;

const FILE_PATH: &str = "./resources/2024_13.txt";

use eqsolver::{
    multivariable::MultiVarNewton,
    nalgebra::{Matrix2, Vector2},
};
use regex::Regex;

const A_TOKEN_PRICE: isize = 3;
const B_TOKEN_PRICE: isize = 1;

/// Solution for https://adventofcode.com/2024/day/13 - Part One & Two.
/// Run by `cargo run --bin 2024_13`.
fn main() {
    // Read and parse file
    let file_content = fs::read_to_string(FILE_PATH).unwrap();
    let machines = parse_file(&file_content);

    // Calculate and print - base token price
    let token_price = get_token_price(&machines);
    println!("Token price - base: {}", token_price);

    // Calculate and print - token price with increased distance
    let mut machines_increased_distance = machines.clone();
    for machine in machines_increased_distance.iter_mut() {
        machine.prize_coords.0 += 10000000000000;
        machine.prize_coords.1 += 10000000000000;
    }
    let token_price = get_token_price(&machines_increased_distance);

    println!("Token price - increased distance: {}", token_price);
}

fn get_token_price(machines: &Vec<Machine>) -> isize {
    let mut token_price = 0;

    for machine in machines {
        // Solve as equations with two variables

        // Convert values to f64 for Newton method algorithm
        let a = (
            f64::value_from(machine.a_button.0).unwrap(),
            f64::value_from(machine.a_button.1).unwrap(),
        );
        let b = (
            f64::value_from(machine.b_button.0).unwrap(),
            f64::value_from(machine.b_button.1).unwrap(),
        );
        let prize_coords = (
            f64::value_from(machine.prize_coords.0).unwrap(),
            f64::value_from(machine.prize_coords.1).unwrap(),
        );

        // Prepare vectorial function and its Jacobian matrix
        let function = |v: Vector2<f64>| {
            Vector2::new(
                v[0] * a.0 + v[1] * b.0 - prize_coords.0,
                v[0] * a.1 + v[1] * b.1 - prize_coords.1,
            )
        };

        let jacobian = |_: Vector2<f64>| Matrix2::new(a.0, b.0, a.1, b.1);

        // Get solution
        let solution = MultiVarNewton::new(function, jacobian)
            .with_tol(1e-4) // Tolerance must be set to 1e-4, otherwise some of the equations with increased prize distance return a SolverError::MaxIterReached
            .solve(Vector2::new(1., 1.))
            .unwrap();

        // Skip non-integer solutions
        let is_integer = |val: &f64| val.fract() < 1e-4 || val.fract() > (1. - 1e-4);

        let a_result = solution.get(0).unwrap();
        let b_result = solution.get(1).unwrap();

        if !is_integer(a_result) || !is_integer(b_result) {
            continue;
        }

        // Skip solutions with negative numbers
        let a_count = a_result.round() as isize;
        let b_count = b_result.round() as isize;

        if a_count < 0 || b_count < 0 {
            continue;
        }

        // Increase token price sum
        token_price += a_count * A_TOKEN_PRICE + b_count * B_TOKEN_PRICE;
    }

    token_price
}

fn parse_file(content: &str) -> Vec<Machine> {
    let mut machines: Vec<Machine> = vec![];

    #[derive(PartialEq)]
    enum Part {
        ButtonA,
        ButtonB,
        Prize,
        EmptyLine,
    }

    let mut machine_helper = Machine {
        a_button: (0, 0),
        b_button: (0, 0),
        prize_coords: (0, 0),
    };
    let mut part = Part::ButtonA;

    for line in content.lines() {
        if part == Part::EmptyLine {
            part = Part::ButtonA;
            continue;
        }

        let number_regex = Regex::new(r"[0-9]+").unwrap();
        let numbers: Vec<u64> = number_regex
            .find_iter(line)
            .map(|val| val.as_str().parse().unwrap())
            .collect();

        match part {
            Part::ButtonA => {
                machine_helper.a_button.0 = numbers[0];
                machine_helper.a_button.1 = numbers[1];

                part = Part::ButtonB;
            }
            Part::ButtonB => {
                machine_helper.b_button.0 = numbers[0];
                machine_helper.b_button.1 = numbers[1];

                part = Part::Prize;
            }
            Part::Prize => {
                machine_helper.prize_coords.0 = numbers[0];
                machine_helper.prize_coords.1 = numbers[1];

                machines.push(machine_helper.clone());

                part = Part::EmptyLine;
            }
            Part::EmptyLine => panic!("Should not get here"),
        }
    }

    machines
}

#[derive(Clone)]
struct Machine {
    a_button: (u64, u64),
    b_button: (u64, u64),
    prize_coords: (u64, u64),
}
