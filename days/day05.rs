use crate::return_sol;
use crate::utils;

use std::collections::VecDeque;

pub fn part1(lines: &Vec<String>) -> utils::Solution {
    let temp = utils::split_vector(lines.clone());

    let mut stacks = utils::parse_stacks(&temp[0]);
    let instructions = utils::parse_instructions(&temp[1]);

    for instruction in instructions {
        for _i in 0..instruction[0] {
            let val = stacks[instruction[1] - 1].pop_back().unwrap();
            stacks[instruction[2] - 1].push_back(val);
        }
    }

    let mut out: Vec<char> = vec![];

    for stack in stacks {
        out.push(stack[stack.len() - 1]);
    }

    return_sol!(String::from_iter(out).as_str())
}
pub fn part2(lines: &Vec<String>) -> utils::Solution {
    let temp = utils::split_vector(lines.clone());

    let mut stacks = utils::parse_stacks(&temp[0]);
    let instructions = utils::parse_instructions(&temp[1]);

    for instruction in instructions {
        let mut temp_stack: VecDeque<char> = VecDeque::new();
        for _i in 0..instruction[0] {
            temp_stack.push_back(stacks[instruction[1] - 1].pop_back().unwrap());
        }
        for (i, char) in temp_stack.iter().enumerate().rev() {
            stacks[instruction[2] - 1].push_back(*char);
        }
    }

    let mut out: Vec<char> = vec![];

    for stack in stacks {
        out.push(stack[stack.len() - 1]);
    }

    return_sol!(String::from_iter(out).as_str())
}
