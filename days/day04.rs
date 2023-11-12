use crate::return_sol;
use crate::utils;
pub fn part1(lines: &Vec<String>) -> utils::Solution {
    let mut total = 0;

    for line in lines {
        let bounds = utils::process_string(line.clone());
        if bounds[0].lower >= bounds[1].lower {
            if bounds[0].upper <= bounds[1].upper {
                total += 1;
                continue;
            }
        }
        if bounds[0].lower <= bounds[1].lower {
            if bounds[0].upper >= bounds[1].upper {
                total += 1;
                continue;
            }
        }
    }
    return_sol!(total)
}
pub fn part2(lines: &Vec<String>) -> utils::Solution {
    let mut total = 0;

    for line in lines {
        let bounds = utils::process_string(line.clone());
        if bounds[0].lower > bounds[1].upper {
            continue;
        }
        if bounds[0].upper < bounds[1].lower {
            continue;
        }
        total += 1;
    }
    return_sol!(total)
}
