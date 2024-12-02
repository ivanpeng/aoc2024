use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::{zip};
use std::num;
use std::path::absolute;
use std::vec::IntoIter;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    fn split_to_two_sorted_lists<R: BufRead>(reader: R) -> Zip<IntoIter<i32>, IntoIter<i32>> {
        let mut l1: Vec<i32> = Vec::new();
        let mut l2: Vec<i32> = Vec::new();
        for line in reader.lines() {
            let line = line.unwrap();
            let values = line.split("   ").collect::<Vec<&str>>();
            l1.push(values[0].parse::<i32>().unwrap());
            l2.push(values[1].parse::<i32>().unwrap());
        }
        l1.sort();
        l2.sort();
        zip(l1, l2)
    }
    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let zipped_list = split_to_two_sorted_lists(reader);
        let mut acc: u32 = 0;
        for zip_value in zipped_list {
            acc += zip_value.0.abs_diff(zip_value.1)
        }
        Ok(usize::try_from(acc)?)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
