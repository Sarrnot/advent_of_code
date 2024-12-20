use std::{cmp::Ordering, fs};

const FILE_PATH: &str = "./resources/2024_07.txt";

/// Solution for https://adventofcode.com/2024/day/7 - Part One & Two.
/// Run by `cargo run --bin 2024_07`.
fn main() {
    // Read and parse file
    let file_content = fs::read_to_string(FILE_PATH).unwrap();
    let rows = parse_file(&file_content);

    // Test operator combinations - no concatenation
    let mut valid_sum_a = 0;
    let allowed_operators_a = vec![Operator::Plus, Operator::Multiply];
    for row in &rows {
        let is_valid = test_operators(
            &Operator::Plus,
            &row.values,
            0,
            row.result,
            &allowed_operators_a,
        );
        if !is_valid {
            continue;
        };
        valid_sum_a += row.result;
    }

    // Test operator combinations - with concatenation
    let mut valid_sum_b = 0;
    let allowed_operators_b = vec![Operator::Plus, Operator::Multiply, Operator::Concatenate];
    for row in &rows {
        let is_valid = test_operators(
            &Operator::Plus,
            &row.values,
            0,
            row.result,
            &allowed_operators_b,
        );
        if !is_valid {
            continue;
        };
        valid_sum_b += row.result;
    }

    // Print result
    println!("Total calibration result: {}", valid_sum_a);
    println!(
        "Total calibration result with concatenation: {}",
        valid_sum_b
    );
}

/// Recursively test different combinations of operators.
fn test_operators(
    operator: &Operator,
    values: &Vec<usize>,
    prev_value: usize,
    expected_result: usize,
    allowed_operators: &Vec<Operator>,
) -> bool {
    // Evaluate on last "iteration"
    if values.len() == 0 {
        match prev_value.cmp(&expected_result) {
            Ordering::Equal => return true,
            _ => return false,
        }
    }

    // Calculate
    let current_result = apply_operator(prev_value, &operator, values[0]);

    // No need to fork if we already exceeded the expected result.
    if current_result > expected_result {
        return false;
    }

    // Fork with operator variants
    let mut sub_values = values.clone();
    sub_values.remove(0);

    for operator in allowed_operators {
        if test_operators(
            operator,
            &sub_values,
            current_result,
            expected_result,
            allowed_operators,
        ) {
            return true;
        }
    }

    return false;
}

fn apply_operator(val1: usize, operator: &Operator, val2: usize) -> usize {
    match operator {
        Operator::Plus => val1 + val2,
        Operator::Multiply => val1 * val2,
        Operator::Concatenate => {
            let val2_digit_length = val2.ilog10() + 1;
            let base: usize = 10;
            val1 * (base.pow(val2_digit_length)) + val2
        }
    }
}

fn parse_file(content: &str) -> Vec<Row> {
    let mut rows = vec![];

    for line in content.lines() {
        let mut parts = line.split(": ");
        let result = parts
            .next()
            .expect("Result missing.")
            .parse::<usize>()
            .expect("Invalid result value.");
        let values: Vec<usize> = parts
            .next()
            .expect("Values missing.")
            .split(" ")
            .map(|val| val.parse::<usize>().expect("Invalid value."))
            .collect();

        rows.push(Row { result, values });
    }

    rows
}

#[derive(Clone)]
struct Row {
    result: usize,
    values: Vec<usize>,
}

#[derive(Clone)]
enum Operator {
    Plus,
    Multiply,
    Concatenate,
}
