use std::{collections::HashMap, fs, vec};

const FILE_PATH: &str = "./resources/2024_05.txt";

/// Solution for https://adventofcode.com/2024/day/5 - Part One & Two.
/// Run by `cargo run --bin 2024_05`.
fn main() -> Result<(), String> {
    // Read file
    let file_content = match fs::read_to_string(FILE_PATH) {
        Ok(text) => text,
        Err(err) => return Err(err.to_string()),
    };

    // Parse file
    let (rules, updates) = parse_file(&file_content);

    // Grouped rules [greater => [smaller, smaller, ...]]
    let grouped_rules = create_grouped_rules(rules);

    // Solve part 1 & 2
    let mut middle_sum_valid: u32 = 0; // only originally valid updates
    let mut middle_sum_invalid: u32 = 0; // sorted (originally invalid) updates

    for update in updates {
        if is_valid_update(&update, &grouped_rules) {
            let middle_value = update[update.len() / 2] as u32;
            middle_sum_valid += middle_value;
        } else {
            let sorted_update = sort_invalid_update(&update, &grouped_rules);
            middle_sum_invalid += sorted_update[sorted_update.len() / 2] as u32;
        }
    }

    // Print result
    println!("Result Part One: {}", middle_sum_valid);
    println!("Result Part Two: {}", middle_sum_invalid);

    Ok(())
}

fn parse_file(content: &str) -> (Vec<[u8; 2]>, Vec<Vec<u8>>) {
    let mut rules = vec![];
    let mut updates = vec![];

    let mut is_first_part = true;

    for (i, line) in content.lines().enumerate() {
        if line.is_empty() {
            is_first_part = false;
            continue;
        }

        let line_num = i + 1;

        let parse_line = |separator: &str| {
            line.split(separator)
                .map(|raw_value| {
                    raw_value
                        .parse::<u8>()
                        .expect(format!("Failed to parse value on line {line_num}.").as_str())
                })
                .collect::<Vec<u8>>()
        };

        if is_first_part {
            let rule = parse_line("|");
            let validated_rule: [u8; 2] = rule
                .try_into()
                .expect(format!("Error on line {line_num}, exactly two values expected.").as_str());
            rules.push(validated_rule);
        } else {
            let update = parse_line(",");
            assert!(
                update.len() % 2 == 1,
                "Error on line {line_num}, update must be odd otherwise there is no middle."
            );
            updates.push(update);
        }
    }

    (rules, updates)
}

fn create_grouped_rules(rules: Vec<[u8; 2]>) -> HashMap<u8, Vec<u8>> {
    let mut grouped_rules: HashMap<u8, Vec<u8>> = HashMap::new();

    // [greater => [smaller, smaller, ...]]
    for [smaller, greater] in rules {
        match grouped_rules.get_mut(&greater) {
            Some(smaller_list) => {
                smaller_list.push(smaller);
            }
            None => {
                grouped_rules.insert(greater, vec![smaller]);
            }
        }
    }

    grouped_rules
}

fn is_valid_update(update: &Vec<u8>, grouped_rules: &HashMap<u8, Vec<u8>>) -> bool {
    // Test each value whether the values to its right don't violate the rules
    for (i, current_value) in update.into_iter().enumerate() {
        let values_to_right = &update[i + 1..update.len()];
        let smaller_list = grouped_rules.get(&current_value).unwrap();

        for tested_value in values_to_right {
            if smaller_list.contains(tested_value) {
                return false;
            }
        }
    }

    true
}

fn sort_invalid_update(original_update: &Vec<u8>, grouped_rules: &HashMap<u8, Vec<u8>>) -> Vec<u8> {
    let mut update = original_update.clone();
    let mut sorted_update = vec![];

    'outer: while update.len() > 0 {
        for (i, current_value) in (&update).into_iter().enumerate() {
            let mut is_value_valid = true;
            let values_to_right = &update[i + 1..update.len()];
            let smaller_list = grouped_rules.get(&current_value).unwrap();

            for tested_value in values_to_right {
                if smaller_list.contains(tested_value) {
                    is_value_valid = false;
                }
            }

            if is_value_valid {
                let value = update.remove(i);
                sorted_update.push(value);
                continue 'outer;
            }
        }

        panic!("Update can't be sorted.");
    }

    sorted_update
}
