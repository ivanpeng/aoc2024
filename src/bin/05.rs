use std::collections::{HashMap, HashSet, VecDeque, BTreeSet};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut forward_graph = HashMap::<u32, BTreeSet<u32>>::new();
        let mut backward_graph = HashMap::<u32, BTreeSet<u32>>::new();
        let mut acc: usize = 0;
        for line in reader.lines() {
            let line = line?;
            if line == String::from("") {
                dbg!(&forward_graph);
                dbg!(&backward_graph);
                continue
            }
            else if line.contains("|") {
                let pages = line.split("|").map(|x| x.parse::<u32>()).collect::<Result<Vec<u32>, _>>()?;
                if pages.len() == 2 {
                    forward_graph.entry(pages[0]).or_insert_with(BTreeSet::new).insert(pages[1]);
                    backward_graph.entry(pages[1]).or_insert_with(BTreeSet::new).insert(pages[0]);
                }
            } else {
                // graph is parsed out
                let mut should_acc: bool = true;
                let nums = line.split(',').map(|x| x.parse::<u32>()).collect::<Result<Vec<u32>, _>>()?;
                for i in 0..nums.len() {
                    let mut is_in_order: bool = true;
                    let slice = nums[i+1..].iter();
                    // Todo: make functional
                    for s in slice {
                        match forward_graph.get(&nums[i]) {
                            Some(set) => {
                                if !set.contains(s) {
                                    is_in_order = false;
                                    break
                                }
                            }
                            _ => {}
                        }
                        match backward_graph.get(&nums[i]) {
                            Some(set) => {
                                if set.contains(s) {
                                    is_in_order = false;
                                    break
                                }
                            }
                            _ => {}
                        }
                    }
                    if !is_in_order {
                        // a number was found to not be in order. cascade break
                        should_acc = false;
                        break
                    }
                }
                if should_acc {
                    acc += nums[nums.len()/2] as usize
                }
            }

        }
        Ok(acc)
    }

    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn compute_order(
        forward: &HashMap<u32, BTreeSet<u32>>,
        backward: &HashMap<u32, BTreeSet<u32>>,
        elements: Vec<u32>,
    ) -> Result<Vec<u32>> {
        let mut graph: HashMap<u32, HashSet<u32>> = HashMap::new();
        let mut in_degree: HashMap<u32, usize> = HashMap::new();

        let mut add_edge = |from: u32, to: u32| {
            graph.entry(from).or_default().insert(to);
            *in_degree.entry(to).or_default() += 1;
            in_degree.entry(from).or_default();
        };

        // Add forward dependencies
        for (&key, values) in forward {
            for &value in values {
                add_edge(value, key);
            }
        }

        // Add backward dependencies
        for (&key, values) in backward {
            for &value in values {
                add_edge(key, value);
            }
        }

        // Ensure all elements are in the in-degree map
        for &element in &elements {
            in_degree.entry(element).or_default();
        }

        let mut queue: VecDeque<u32> = in_degree
            .iter()
            .filter(|(_, &deg)| deg == 0)
            .map(|(&node, _)| node)
            .collect();

        let mut sorted_order = Vec::new();

        // Process nodes with zero in-degree
        while let Some(node) = queue.pop_front() {
            sorted_order.push(node);

            if let Some(neighbors) = graph.get(&node) {
                for &neighbor in neighbors {
                    let entry = in_degree.get_mut(&neighbor).unwrap();
                    *entry -= 1;
                    if *entry == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        // Add any leftover nodes that are in the input elements but not yet sorted
        let element_set: HashSet<u32> = elements.iter().cloned().collect();
        let leftover_nodes: Vec<u32> = element_set
            .difference(&sorted_order.iter().cloned().collect())
            .cloned()
            .collect();

        sorted_order.extend(leftover_nodes);

        // Final filtering to match input elements order
        sorted_order.retain(|x| element_set.contains(x));

        Ok(sorted_order)
    }

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut forward_graph = HashMap::<u32, BTreeSet<u32>>::new();
        let mut backward_graph = HashMap::<u32, BTreeSet<u32>>::new();
        let mut acc: usize = 0;
        for line in reader.lines() {
            let line = line?;
            if line == String::from("") {
                dbg!(&forward_graph);
                dbg!(&backward_graph);
                continue
            }
            else if line.contains("|") {
                let pages = line.split("|").map(|x| x.parse::<u32>()).collect::<Result<Vec<u32>, _>>()?;
                if pages.len() == 2 {
                    forward_graph.entry(pages[0]).or_insert_with(BTreeSet::new).insert(pages[1]);
                    backward_graph.entry(pages[1]).or_insert_with(BTreeSet::new).insert(pages[0]);
                }
            } else {
                // graph is parsed out
                let mut should_acc: bool = true;
                let nums = line.split(',').map(|x| x.parse::<u32>()).collect::<Result<Vec<u32>, _>>()?;
                for i in 0..nums.len() {
                    let mut is_in_order: bool = true;
                    let slice = nums[i+1..].iter();
                    for s in slice {
                        match forward_graph.get(&nums[i]) {
                            Some(set) => {
                                if !set.contains(s) {
                                    is_in_order = false;
                                    break
                                }
                            }
                            _ => {}
                        }
                        match backward_graph.get(&nums[i]) {
                            Some(set) => {
                                if set.contains(s) {
                                    is_in_order = false;
                                    break
                                }
                            }
                            _ => {}
                        }
                    }
                    if !is_in_order {
                        // a number was found to not be in order. cascade break
                        should_acc = false;
                        break
                    }
                }
                if !should_acc {
                    let ordered_list = compute_order(&forward_graph, &backward_graph, nums);
                    dbg!(&ordered_list);
                    if ordered_list.is_ok() {
                        let l = ordered_list.unwrap();
                        acc += l[l.len()/2] as usize;
                    }
                }
            }

        }
        Ok(acc)
    }

    assert_eq!(123, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
