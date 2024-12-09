use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() -> io::Result<()> {
    let path = Path::new("assets/day3.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    
    Ok(())
}
