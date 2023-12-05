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

pub fn part2(lines: &Vec<String>) -> utils::Solution {
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut tot = 0;
    for line in lines {
        let mut pos1: u32 = 0;
        let mut pos2: u32 = 0;

        let mut idx = 0;
        while let Some(slice) = line.get(idx..) {
            if slice.starts_with(|c| c as u32 <= 58) {
                pos1 = slice.chars().nth(0).unwrap() as u32 - 48;
                break;
            } else {
                for (i, num) in numbers.iter().enumerate() {
                    if slice.starts_with(num) {
                        pos1 = i as u32 + 1;
                        break;
                    }
                }
            }
            idx += 1;
            if pos1 > 0 {
                break;
            }
        }
        let mut idx = line.len();
        while let Some(slice) = line.get(..idx) {
            if slice.ends_with(|c| c as u32 <= 58) {
                pos2 = slice.chars().nth(slice.len() - 1).unwrap() as u32 - 48;
                break;
            } else {
                for (i, num) in numbers.iter().enumerate() {
                    if slice.ends_with(num) {
                        pos2 = i as u32 + 1;
                        break;
                    }
                }
            }
            idx -= 1;
            if pos2 > 0 {
                break;
            }
        }
        tot += (pos1 * 10) + pos2;
    }
    return_sol!(tot)
}
