use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("assets/day5.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut pairs: Vec<(i32, i32)> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();
    let mut is_subsequent_section = false;

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            is_subsequent_section = true;
            continue;
        }

        if is_subsequent_section {
            let mut update_vec: Vec<i32> = Vec::new();
            for num in line.split(',') {
                if let Ok(value) = num.trim().parse::<i32>() {
                    update_vec.push(value);
                }
            }
            updates.push(update_vec);
        } else {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 2 {
                let num1 = parts[0].trim().parse::<i32>().unwrap();
                let num2 = parts[1].trim().parse::<i32>().unwrap();
                pairs.push((num1, num2));
            }
        }
    }

    let mut result = 0;

    for update in updates {
        let mut correct = true;
        for pair in &pairs {
            let num1 = pair.0;
            let num2 = pair.1;
            if(update.contains(&num1) && update.contains(&num2)) {
                if(update.iter().position(|&x| x == num1).unwrap() > update.iter().position(|&x| x == num2).unwrap()) {
                    correct = false;
                    break;
                }
            }
        }
        if(correct) {
            let middle_index = update.len() / 2;
            result += update[middle_index];
        }
    }
    println!("Result: {}", result);
    Ok(())
}