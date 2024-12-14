use std::{fs, str::FromStr};

pub fn get_lists_from_file<T: FromStr, const N: usize>(
    file_path: &str,
) -> Result<[Vec<T>; N], String> {
    // Read file
    let file_content = match fs::read_to_string(file_path) {
        Ok(text) => text,
        Err(err) => return Err(err.to_string()),
    };

    // Create empty lists
    let mut lists = [const { Vec::new() }; N];

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

        // Parse and save both values on current line.
        for (j, raw_value) in parts.enumerate() {
            match raw_value.parse::<T>() {
                Ok(value) => {
                    lists[j].push(value);
                }
                Err(_) => {
                    return Err(format!("Invalid value '{raw_value}' on line {line_num}."));
                }
            }
        }
    }

    Ok(lists)
}
