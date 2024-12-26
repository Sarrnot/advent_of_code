use std::{collections::HashMap, fs};

const FILE_PATH: &str = "./resources/2024_11.txt";

/// Solution for https://adventofcode.com/2024/day/11 - Part One & Two.
/// Run by `cargo run --bin 2024_11`.
fn main() {
    // Read and parse file
    let file_content = fs::read_to_string(FILE_PATH).unwrap();
    let stones = parse_file(&file_content);

    // Calculate
    let mut cache: Cache = HashMap::new();

    let mut stone_count_25 = 0;
    for stone in &stones {
        stone_count_25 += get_stone_count(*stone, 0, 25, &mut cache);
    }

    let mut stone_count_75 = 0;
    for stone in &stones {
        stone_count_75 += get_stone_count(*stone, 0, 75, &mut cache);
    }

    // Print result
    println!("Stone count - 25 iterations: {}", stone_count_25);
    println!("Stone count - 75 iterations: {}", stone_count_75);
}

fn get_stone_count(stone_value: usize, depth: usize, max_depth: usize, cache: &mut Cache) -> usize {
    // Reached last "iteration"
    if depth == max_depth {
        return 1;
    }

    // Try to find value in cache
    if let Some(cache_item) = cache.get(&stone_value) {
        if let Some(cached_stone_count) = cache_item.get(&(max_depth - depth)) {
            return *cached_stone_count;
        }
    }

    // Recursively calculate count if not cached
    let mut apply_recursion = |value| get_stone_count(value, depth + 1, max_depth, cache);

    let stone_count = if stone_value == 0 {
        apply_recursion(1)
    } else if get_digit_length_unchecked(stone_value) % 2 == 0 {
        let value_string = stone_value.to_string();
        let (left, right) = value_string.split_at(get_digit_length_unchecked(stone_value) / 2);
        apply_recursion(left.parse().unwrap()) + apply_recursion(right.parse().unwrap())
    } else {
        apply_recursion(stone_value * 2024)
    };

    // Save calculated count to cache
    match cache.get_mut(&stone_value) {
        Some(cache_item) => {
            cache_item.insert(max_depth - depth, stone_count);
        }
        None => {
            cache.insert(
                stone_value,
                HashMap::from([(max_depth - depth, stone_count)]),
            );
        }
    };

    stone_count
}

fn get_digit_length_unchecked(value: usize) -> usize {
    (value.ilog10() + 1).try_into().unwrap()
}

fn parse_file(content: &str) -> Vec<usize> {
    content
        .split(" ")
        .map(|val| val.parse::<usize>().unwrap())
        .collect()
}

type CacheItem = HashMap<usize, usize>;
type Cache = HashMap<usize, CacheItem>;
