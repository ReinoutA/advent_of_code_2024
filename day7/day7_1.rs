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

        if can_satisfy(target, &numbers, 0, numbers[0]) {
            total_calibration_result += target;
        }
    }

    println!("Total Calibration Result: {}", total_calibration_result);
    Ok(())
}

fn can_satisfy(target: i64, numbers: &[i64], index: usize, current_result: i64) -> bool {
    // Base case: if we've used all numbers, check if the result matches the target
    if index == numbers.len() - 1 {
        return current_result == target;
    }

    // Early exit: skip if current_result already exceeds the target
    if current_result > target {
        return false;
    }

    // Recursive case: try both + and * with the next number
    let next_number = numbers[index + 1];
    can_satisfy(target, numbers, index + 1, current_result + next_number)
        || can_satisfy(target, numbers, index + 1, current_result.saturating_mul(next_number))
}