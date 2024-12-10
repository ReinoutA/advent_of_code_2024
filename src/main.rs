use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("assets/day1.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    
    Ok(())
}
