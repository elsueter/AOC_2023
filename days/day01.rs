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
        ("zero", 0u8),
        ("one", 1u8),
        ("two", 2u8),
        ("three", 3u8),
        ("four", 4u8),
        ("five", 5u8),
        ("six", 6u8),
        ("seven", 7u8),
        ("eight", 8u8),
        ("nine", 9u8),
    ]);

    let mut tot: u32 = 0;

    for line in lines {
        let mut i = 0;
        let mut j = 0;
        let mut u8_line: Vec<u8> = vec![];

        while let Some(slice) = line.get(i..j) {
            println!("{:}, {:?}", slice, u8_line);
            if digits.contains_key(slice) {
                u8_line.push(*digits.get(slice).unwrap());
                i = j;
            } else {
                j += 1;
                if j - i >= 6 {
                    u8_line.push(slice.chars().next().unwrap() as u8 - 48);
                    i += 1;
                }

                if j > line.len() {
                    break;
                }
            }
        }
        let pos1 = u8_line.iter().position(|x| x < &58).unwrap();
        let pos2 = u8_line.iter().rposition(|x| x < &58).unwrap();
    }

    return_sol!(tot)
}
