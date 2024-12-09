use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() -> io::Result<()> {
    let path = Path::new("assets/day3.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let content: String = reader.lines().map(|line| line.unwrap()).collect::<Vec<String>>().join("\n");

    let re_instruction = Regex::new(r"(don't\(\))|(do\(\))").unwrap();

    let re_mul = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    let mut result = 0;
    let mut enabled = true;

    let mut last_pos = 0;

    for mat in re_instruction.find_iter(&content) {
        let start_pos = mat.start();
        let end_pos = mat.end();

        let part = &content[last_pos..start_pos];

        if enabled {
            let filtered: Vec<&str> = re_mul.find_iter(part)
                .map(|mat| mat.as_str())
                .collect();

            for numbers in filtered {
                let number_re = Regex::new(r"\d{1,3}").unwrap();
                let numbers = number_re.find_iter(numbers)
                    .map(|mat| mat.as_str())
                    .collect::<Vec<&str>>();

                let num1 = numbers[0].parse::<i32>().unwrap();
                let num2 = numbers[1].parse::<i32>().unwrap();

                result += num1 * num2;
            }
        }

        if &content[start_pos..end_pos] == "don't()" {
            enabled = false;
        } else if &content[start_pos..end_pos] == "do()" {
            enabled = true;
        }

        last_pos = end_pos;
    }

    let part = &content[last_pos..];
    if enabled {
        let filtered: Vec<&str> = re_mul.find_iter(part)
            .map(|mat| mat.as_str())
            .collect();

        for numbers in filtered {
            let number_re = Regex::new(r"\d{1,3}").unwrap();
            let numbers = number_re.find_iter(numbers)
                .map(|mat| mat.as_str())
                .collect::<Vec<&str>>();

            let num1 = numbers[0].parse::<i32>().unwrap();
            let num2 = numbers[1].parse::<i32>().unwrap();

            result += num1 * num2;
        }
    }

    println!("Result: {}", result);

    Ok(())
}
