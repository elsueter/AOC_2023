use crate::return_sol;
use crate::utils;

pub fn part1(lines: &Vec<String>) -> utils::Solution {
    let results = utils::parse_day03_string(lines);
    let nums = results.0;
    let symbols = results.1;
    let mut tot = 0;

    for num in nums.iter() {
        for symbol in symbols.iter() {
            if num.next_to_solution(symbol) {
                tot += num.get_value();
                break;
            }
        }
    }

    return_sol!(tot)
}
pub fn part2(lines: &Vec<String>) -> utils::Solution {
    let results = utils::parse_day03_string(lines);
    let nums = results.0;
    let symbols = results.1;
    let mut tot = 0;

    for symbol in symbols.iter() {
        if symbol.get_value() == '*' as u8 {
            let mut num_tot = 0;
            let mut gear_tot = 1;
            for num in nums.iter() {
                if num.next_to_solution(symbol) {
                    num_tot += 1;
                    gear_tot *= num.get_value();
                }
            }
            if num_tot > 1 {
                tot += gear_tot;
            }
        }
    }

    return_sol!(tot)
}
