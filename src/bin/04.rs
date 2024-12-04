use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn word_search_count(grid: &Vec<String>, word: &str) -> usize {
        let rows = grid.len();
        let cols = grid[0].len();
        let word_chars: Vec<char> = word.chars().collect();
        let directions = vec![
            (0, 1),   // right
            (0, -1),  // left
            (1, 0),   // down
            (-1, 0),  // up
            (1, 1),   // diagonal down-right
            (1, -1),  // diagonal down-left
            (-1, 1),  // diagonal up-right
            (-1, -1), // diagonal up-left
        ];

        let in_bounds = |row: isize, col: isize| row >= 0 && col >= 0 && (row as usize) < rows && (col as usize) < cols;

        let mut acc = 0;

        for row in 0..rows {
            for col in 0..cols {
                for &(dr, dc) in &directions {
                    let mut r = row as isize;
                    let mut c = col as isize;
                    let mut found = true;

                    for &ch in &word_chars {
                        if !in_bounds(r, c) || grid[r as usize].chars().nth(c as usize).unwrap() != ch {
                            found = false;
                            break;
                        }
                        r += dr;
                        c += dc;
                    }

                    if found {
                        acc += 1;
                    }
                }
            }
        }
        acc
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let grid: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
        Ok(word_search_count(&grid, "XMAS"))
    }

    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn word_xmas_count(grid: &Vec<Vec<char>>) -> usize {
        let rows = grid.len();
        let cols = grid[0].len();
        let word_char: char = 'A';

        let mut acc = 0;

        for row in 1..rows - 1 {
            for col in 1..cols - 1 {
                if grid[row][col] == word_char {
                    // search diagonals, guaranteed to be in bounds
                    if (grid[row-1][col-1] == 'M' && grid[row-1][col+1] == 'M' && grid[row+1][col-1] == 'S' && grid[row+1][col+1] == 'S') ||
                        (grid[row-1][col-1] == 'M' && grid[row-1][col+1] == 'S' && grid[row+1][col-1] == 'M' && grid[row+1][col+1] == 'S') ||
                        (grid[row-1][col-1] == 'S' && grid[row-1][col+1] == 'S' && grid[row+1][col-1] == 'M' && grid[row+1][col+1] == 'M') ||
                        (grid[row-1][col-1] == 'S' && grid[row-1][col+1] == 'M' && grid[row+1][col-1] == 'S' && grid[row+1][col+1] == 'M') {
                        acc +=1;
                    }
                }
            }
        }
        acc
    }

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let grid: Vec<Vec<char>> = reader.lines().map(|x| x.unwrap().chars().collect()).collect();
        Ok(word_xmas_count(&grid))
    }

    assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
