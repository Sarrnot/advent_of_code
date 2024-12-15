use std::fs;

use regex::Regex;

const FILE_PATH: &str = "./resources/2024_03.txt";

/// Solution for https://adventofcode.com/2024/day/3 - Part Two.
/// Run by `cargo run --bin 2024_03_b`.
fn main() -> Result<(), String> {
    // Read file
    let file_content = match fs::read_to_string(FILE_PATH) {
        Ok(text) => text,
        Err(err) => return Err(err.to_string()),
    };

    // Prepare regex
    let expression_regex = Regex::new(r"(do\(\))|don't\(\)|mul\([0-9]+,[0-9]+\)").unwrap(); // find `do()` `don't()` or a valid `mul(a,b)`
    let number_regex = Regex::new(r"[0-9]+").unwrap();

    // Find and evaluate expressions
    let mut sum = 0;
    let mut enabled = true;
    let expressions = expression_regex.find_iter(&file_content);

    for expression_match in expressions {
        let expression = expression_match.as_str();

        if expression == "do()" {
            enabled = true;
            continue;
        } else if expression == "don't()" {
            enabled = false;
            continue;
        }

        if !enabled {
            continue;
        }

        let mut number_matches = number_regex.find_iter(expression);

        // Parse first and second number
        let mut numbers = vec![];
        for _ in 0..=1 {
            let number_string = number_matches.next().unwrap().as_str(); // No need to validate whether exists, already validated by regex.
            let number = number_string.parse::<i32>().unwrap(); // Is number - validated already by regex, but will panic when over/underflow.
            numbers.push(number);
        }

        // Add a*b to sum
        sum += numbers[0] * numbers[1];
    }

    // Print result
    println!("Result: {}", sum);

    Ok(())
}
