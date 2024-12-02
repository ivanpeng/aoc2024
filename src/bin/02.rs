use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn is_monotonically_increasing(list: Vec<i32>) -> bool {
        list.windows(2).all(|w| w[0] < w[1] && w[1] - w[0] <= 3)
    }

    fn is_report_safe(report: &Vec<i32>) -> bool {
        let mut report_reversed = report.clone();
        report_reversed.reverse();
        let is_sorted =  report.is_sorted() || report_reversed.is_sorted();
        if !is_sorted {
            return false
        }
        // Know it is sorted at this point, determine monotonic increasing list and determine diff is 1 or 3
        let increasing_report = if report.first() > report.last() { report_reversed } else { report.clone() };
        let is_report_safe = is_monotonically_increasing(increasing_report);
        is_report_safe
    }
    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut acc: usize = 0;
        for line in reader.lines() {
            let line = line?;
            let values: Vec<i32> = line.split(' ').filter_map(|x | i32::from_str(x).ok()).collect();
            if is_report_safe(&values) { acc += 1; }
        }
        Ok(acc)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn pop_out_by_index(vec: &Vec<i32>, index: usize) -> Vec<i32> {
        let mut vec = vec.clone();
        vec.drain(index..=index);
        vec
    }

    fn is_report_safe_with_dampener(report: Vec<i32>) -> bool {
        if is_report_safe(&report) {
            return true
        }
        // iterate through each element in report, removing it, and checking
        for i in 0..report.len() {
            let r = pop_out_by_index(&report, i);
            if is_report_safe(&r) {
                return true
            }
        }
        false
    }
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut acc: usize = 0;
        for line in reader.lines() {
            let line = line?;
            let values: Vec<i32> = line.split(' ').filter_map(|x | i32::from_str(x).ok()).collect();
            if is_report_safe_with_dampener(values) { acc += 1; }
        }
        Ok(acc)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
