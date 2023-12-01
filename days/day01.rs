use crate::return_sol;
use crate::utils;
pub fn part1(lines: &Vec<String>) -> utils::Solution {
    let mut tot: u32 = 0;
    for line in lines {
        let u8_line = line.as_bytes();

        let pos1 = u8_line.iter().position(|x| x < &58).unwrap();
        let pos2 = u8_line.iter().rposition(|x| x < &58).unwrap();

        tot += u32::from((u8_line[pos1] - 48) * 10) + u32::from(u8_line[pos2] - 48);
    }
    return_sol!(tot)
}

use std::collections::HashMap;

pub fn part2(lines: &Vec<String>) -> utils::Solution {
    let digits = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let mut tot: u32 = 0;

    for line in lines {
        let mut new_line = line.to_string();

        let mut lowest = "";
        let mut highest = "";
        let mut index = 1000;
        let mut rindex = 0;

        for digit in digits.keys() {
            if let Some(var) = line.find(digit) {
                if index > var {
                    index = var;
                    lowest = digit;
                }
            }
            if let Some(var) = line.rfind(digit) {
                if rindex < var {
                    rindex = var;
                    highest = digit;
                }
            }
        }

        if lowest != "" {
            new_line = line.replacen(lowest, digits.get(lowest).unwrap(), 1);
        }
        if highest != "" {
            new_line = new_line.replace(highest, digits.get(highest).unwrap());
        }

        let u8_line = new_line.as_bytes();

        let pos1 = u8_line.iter().position(|x| x < &58).unwrap();
        let pos2 = u8_line.iter().rposition(|x| x < &58).unwrap();

        let cur = u32::from((u8_line[pos1] - 48) * 10) + u32::from(u8_line[pos2] - 48);
        tot += cur;
    }

    return_sol!(tot)
}
