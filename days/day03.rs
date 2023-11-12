use crate::return_sol;
use crate::utils;
pub fn part1(lines: &Vec<String>) -> utils::Solution {
    let mut total = 0;

    for line in lines {
        let first = &line[..line.chars().count() / 2];
        let second = &line[line.chars().count() / 2..];
        let mut seen_values = [0; 52];
        for c in first.chars() {
            let mut value = c as usize;
            if value > 96 {
                value -= 97;
            } else {
                value -= 39;
            }

            seen_values[value] += 1;
        }
        for c in second.chars() {
            let mut value = c as usize;
            if value > 96 {
                value -= 97;
            } else {
                value -= 39;
            }

            if seen_values[value] > 0 {
                total += value + 1;
                break;
            }
        }
    }
    return_sol!(total)
}
pub fn part2(lines: &Vec<String>) -> utils::Solution {
    let mut seen_values = [0; 52];
    let mut total = 0;
    let mut mode = 0;

    for line in lines {
        if mode % 3 == 0 {
            for (i, value) in seen_values.iter().enumerate() {
                if *value == 3 {
                    total += i + 1;
                }
            }
            seen_values = [0; 52];
        }
        for c in line.chars() {
            let mut value = c as usize;
            if value > 96 {
                value -= 97;
            } else {
                value -= 39;
            }

            if seen_values[value] == 0 + mode % 3 {
                seen_values[value] += 1;
            }
        }
        mode += 1;
    }
    for (i, value) in seen_values.iter().enumerate() {
        if *value == 3 {
            total += i + 1;
        }
    }
    return_sol!(total)
}
