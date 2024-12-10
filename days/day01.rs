use crate::return_sol;
use crate::utils;
pub fn part1(lines: &Vec<String>) -> utils::Solution {
    let mut lhs: Vec<u32> = vec![];
    let mut rhs: Vec<u32> = vec![];
    for line in lines {
        let mut split_line = line.split("   ");
        lhs.push(split_line.next().unwrap().parse::<u32>().unwrap());
        rhs.push(split_line.next().unwrap().parse::<u32>().unwrap());

        lhs.sort();
        rhs.sort();
    }

    let mut result = 0;
    lhs.iter()
        .enumerate()
        .for_each(|(i, x)| result += u32::abs_diff(*x, rhs[i]));
    return_sol!(result)
}
pub fn part2(lines: &Vec<String>) -> utils::Solution {
    let mut lhs: Vec<u32> = vec![];
    let mut rhs: Vec<u32> = vec![];
    for line in lines {
        let mut split_line = line.split("   ");
        lhs.push(split_line.next().unwrap().parse::<u32>().unwrap());
        rhs.push(split_line.next().unwrap().parse::<u32>().unwrap());
    }

    let mut result = 0;

    lhs.iter()
        .for_each(|num| result += num * (rhs.iter().filter(|x| *x == num).count() as u32));

    return_sol!(result)
}
