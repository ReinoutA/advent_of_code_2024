use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("assets/day7.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut total_calibration_result = 0;

    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split_whitespace();

        // Parse the target value
        let target = if let Some(target_str) = parts.next() {
            target_str.trim_end_matches(':').parse::<i64>().unwrap()
        } else {
            continue;
        };

        let numbers: Vec<i64> = parts.map(|num| num.parse::<i64>().unwrap()).collect();

        if can_satisfy_with_operators(target, &numbers, 0, numbers[0]) {
            total_calibration_result += target;
        }
    }

    println!("Total Calibration Result: {}", total_calibration_result);
    Ok(())
}

fn can_satisfy_with_operators(target: i64, numbers: &[i64], index: usize, current_result: i64) -> bool {
    // Base case: if we've used all numbers, check if the result matches the target
    if index == numbers.len() - 1 {
        return current_result == target;
    }

    // Recursive case: try +, *, and concatenation ||
    let next_number = numbers[index + 1];

    // Try addition
    if can_satisfy_with_operators(target, numbers, index + 1, current_result + next_number) {
        return true;
    }

    // Try multiplication
    if let Some(product) = current_result.checked_mul(next_number) {
        if can_satisfy_with_operators(target, numbers, index + 1, product) {
            return true;
        }
    }

    // Try concatenation (||)
    let concatenated = concat_numbers(current_result, next_number);
    if concatenated <= i64::MAX as i64 && can_satisfy_with_operators(target, numbers, index + 1, concatenated as i64) {
        return true;
    }

    false
}

fn concat_numbers(left: i64, right: i64) -> i64 {
    let left_str = left.to_string();
    let right_str = right.to_string();
    let concatenated_str = format!("{}{}", left_str, right_str);
    concatenated_str.parse::<i64>().unwrap()
}