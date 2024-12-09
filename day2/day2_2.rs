use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

const MAX_DIFFERENCE: i32 = 3;
const MIN_DIFFERENCE: i32 = 1;

fn is_strictly_ascending(vec: &Vec<i32>) -> bool {
    vec.windows(2).all(|w| w[0] < w[1] && (w[0] - w[1]).abs() >= MIN_DIFFERENCE && (w[0] - w[1]).abs() <= MAX_DIFFERENCE)
}

fn is_strictly_descending(vec: &Vec<i32>) -> bool {
    vec.windows(2).all(|w| w[0] > w[1] && (w[0] - w[1]).abs() >= MIN_DIFFERENCE && (w[0] - w[1]).abs() <= MAX_DIFFERENCE)
}

fn main() -> io::Result<()> {
    let path = Path::new("assets/day2.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut num_safe_reports : i32 = 0;

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();

        for i in 0..numbers.len() {
            let mut modified_vec = numbers.to_vec();
            modified_vec.remove(i);
            if is_strictly_descending(&modified_vec) || is_strictly_ascending(&modified_vec) {
                num_safe_reports += 1;
                break;
            }
        }
    }

    println!("Number of safe reports: {}", num_safe_reports);
    Ok(())
}