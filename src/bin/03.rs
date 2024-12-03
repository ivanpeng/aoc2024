use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Mul};
use std::str::FromStr;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use adv_code_2024::*;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;
        let mut acc: usize = 0;
        for line in reader.lines() {
            let line = line?;
            for captures in re.captures_iter(&*line) {
                let x = i32::from_str(captures.get(1).unwrap().as_str())?;
                let y = i32::from_str(captures.get(2).unwrap().as_str())?;
                acc += x.mul(y) as usize;
            }
        }
        Ok(acc)
    }

    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut acc: usize = 0;
        let key: Vec<&str> =vec!["do()", "don't()"];
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;
        let mut contents = reader.lines().collect::<Result<Vec<_>, _>>()?.join("\n");
        let mut substr = contents;
        let mut should_extract: bool = true;
        while substr.len() > 0 {
            let start: usize = 0;
            let end = substr.find(key[should_extract as usize]).unwrap_or(substr.len());

            let sequence = &substr[start..end];
            // dbg!("substr: {}, sequence: {}, should_extract: {}", &substr, &sequence, should_extract);
            if should_extract {
                for captures in re.captures_iter(sequence) {
                    let x = i32::from_str(captures.get(1).unwrap().as_str())?;
                    let y = i32::from_str(captures.get(2).unwrap().as_str())?;
                    acc += x.mul(y) as usize;
                }
            }
            // shrink substr and flip the switch
            let start = if end.add(key[should_extract as usize].len()) >= substr.len() { substr.len() } else { end.add(key[should_extract as usize].len()) };
            substr = substr[start..substr.len()].parse()?;
            should_extract = !should_extract;

        }
        Ok(acc)
    }

    assert_eq!(48, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
