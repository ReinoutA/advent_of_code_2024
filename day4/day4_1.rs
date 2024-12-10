use std::fs::File;
use std::io::{self, BufRead};

const DIRECTIONS: [(isize, isize); 8] = [
    (0, 1),  // Right
    (0, -1), // Left
    (1, 0),  // Down
    (-1, 0), // Up
    (1, 1),  // Down-Right
    (-1, -1),// Up-Left
    (1, -1), // Down-Left
    (-1, 1), // Up-Right
];

fn find_word_count(grid: &Vec<Vec<char>>, word: &str) -> usize {
    let word_chars: Vec<char> = word.chars().collect();
    let word_len = word.len();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut count = 0;

    // Iterate over each cell as the starting point
    for r in 0..rows {
        for c in 0..cols {
            // Check for word in all 8 directions
            for &(dr, dc) in &DIRECTIONS {
                let mut found = true;

                for i in 0..word_len {
                    let nr = r as isize + i as isize * dr;
                    let nc = c as isize + i as isize * dc;

                    if nr < 0 || nr >= rows as isize || nc < 0 || nc >= cols as isize {
                        found = false;
                        break;
                    }

                    if grid[nr as usize][nc as usize] != word_chars[i] {
                        found = false;
                        break;
                    }
                }

                if found {
                    count += 1;
                }
            }
        }
    }

    count
}

fn read_grid_from_file(filename: &str) -> io::Result<Vec<Vec<char>>> {
    let mut grid = Vec::new();

    if let Ok(lines) = File::open(filename).map(io::BufReader::new).map(|reader| reader.lines()) {
        for line in lines {
            let line = line?;
            let row: Vec<char> = line.chars().collect();
            grid.push(row);
        }
    }

    Ok(grid)
}

fn main() -> io::Result<()> {
    let filename = "assets/day4.txt";
    let word = "XMAS";

    let grid = read_grid_from_file(filename)?;
    let count = find_word_count(&grid, word);

    println!("The word '{}' appears {} times in the grid.", word, count);

    Ok(())
}
