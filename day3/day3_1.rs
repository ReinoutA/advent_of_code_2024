use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use regex::Regex;

fn main() -> io::Result<()> {
    let path = Path::new("assets/day3.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let content : String = reader.lines().map(|line| line.unwrap()).collect();

    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    let filtered: Vec<&str> = re.find_iter(&content)
        .map(|mat| mat.as_str())
        .collect();

    let mut result : i32 = 0;
    for numbers in filtered {
        let re = Regex::new(r"\d{1,3}").unwrap();
        let numbers = re.find_iter(numbers)
            .map(|mat| mat.as_str())
            .collect::<Vec<&str>>();

        let num1 = numbers[0].parse::<i32>().unwrap();
        let num2 = numbers[1].parse::<i32>().unwrap();

        result += num1 * num2;
    }

    println!("Result: {}", result);
    Ok(())
}