use crate::return_sol;
use crate::utils;
use regex::Regex;

pub fn part1(lines: &Vec<String>) -> utils::Solution {
    let mut result = 0;
    let regex = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    for line in lines {
        let results: Vec<&str> = regex.find_iter(line).map(|m| m.as_str()).collect();
        for instr in results {
            let num_regex = Regex::new(r"[0-9]+").unwrap();
            let nums: Vec<u32> = num_regex
                .find_iter(instr)
                .map(|m| m.as_str().parse::<u32>().unwrap())
                .collect();
            result += nums[0] * nums[1];
        }
    }
    return_sol!(result)
}
pub fn part2(lines: &Vec<String>) -> utils::Solution {
    let mut result = 0;
    let regex = Regex::new(r"mul\([0-9]+,[0-9]+\)|don't\(\)|do\(\)").unwrap();
    let mut enabled = true;
    for line in lines {
        let results: Vec<&str> = regex.find_iter(line).map(|m| m.as_str()).collect();
        for instr in results {
            match instr.get(..3) {
                Some("don") => enabled = false,
                Some("do(") => enabled = true,
                Some("mul") => {
                    if enabled {
                        let results: Vec<&str> =
                            regex.find_iter(line).map(|m| m.as_str()).collect();
                        let num_regex = Regex::new(r"[0-9]+").unwrap();
                        let nums: Vec<u32> = num_regex
                            .find_iter(instr)
                            .map(|m| m.as_str().parse::<u32>().unwrap())
                            .collect();
                        result += nums[0] * nums[1];
                    }
                }
                _ => println!("err"),
            }
        }
    }
    return_sol!(result)
}
