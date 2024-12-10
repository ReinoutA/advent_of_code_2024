use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

const DIAGONAL_DIRECTIONS: [(isize, isize); 4] = [
    (1, 1),  // Down-Right
    (-1, -1),// Up-Left
    (1, -1), // Down-Left
    (-1, 1), // Up-Right
];

fn find_word_count(grid: &Vec<Vec<char>>, word: &str) -> isize {
    let word_chars: Vec<char> = word.chars().collect();
    let word_len = word.len();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut count = 0;

    let mut middle_points: HashMap<(isize, isize), isize> = HashMap::new();

    // Iterate over each cell as the starting point
    for r in 0..rows {
        for c in 0..cols {
            // Check for word in all 4 DIAGONAL_DIRECTIONS
            for &(dr, dc) in &DIAGONAL_DIRECTIONS {
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
                    let middle_index = word_len / 2;
                    let middle_r = r as isize + middle_index as isize * dr;
                    let middle_c = c as isize + middle_index as isize * dc;
                    let new_count = middle_points.get(&(middle_r, middle_c)).unwrap_or(&0) + 1;
                    middle_points.insert((middle_r, middle_c), new_count);
                }
            }
        }
    }

    for (_, &value) in middle_points.iter() {
        if value > 1 {
            count += 1;
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
    let word = "MAS";

    let grid = read_grid_from_file(filename)?;
    let count = find_word_count(&grid, word);

    println!("The word '{}' appears {} times in the grid.", word, count);

    Ok(())
}
