use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("assets/day1.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut columns = line.split_whitespace();
        if let (Some(col1), Some(col2)) = (columns.next(), columns.next()) {
            vec1.push(col1.parse::<i32>().unwrap());
            vec2.push(col2.parse::<i32>().unwrap());
        }
    }

    let mut total_similarity = 0;

    // for i in 0..vec1.len() {
    //     let value = vec1[i];
    //     let count = vec2.iter().filter(|&&x| x == value).count() as i32;
    //     total_similarity += value * count;
    // }

    total_similarity = vec1.iter()
        .map(|&value| value * vec2.iter().filter(|&&x| x == value).count() as i32)
        .sum();

    println!("Total similarity: {}", total_similarity);
    Ok(())
}
