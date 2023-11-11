use crate::utils;
use std::collections::VecDeque;

pub fn part_1(lines: &Vec<String>) -> i32{
    let temp = utils::split_vector(lines.clone());

    let mut stacks = utils::parse_stacks(&temp[0]);
    let instructions = utils::parse_instructions(&temp[1]);

    for instruction in instructions{
        for _i in 0..instruction[0]{
            let val = stacks[instruction[1]-1].pop_back().unwrap();
            stacks[instruction[2]-1].push_back(val);
        }
    }

    let mut out: Vec<char> = vec![];

    for stack in stacks{
        out.push(stack[stack.len()-1]);
    }

    String::from_iter(out).parse::<i32>().unwrap()
}

pub fn part_2(lines: &Vec<String>) -> i32{
    let temp = utils::split_vector(lines.clone());

    let mut stacks = utils::parse_stacks(&temp[0]);
    let instructions = utils::parse_instructions(&temp[1]);

    for instruction in instructions{
        let mut temp_stack: VecDeque<char> = VecDeque::new();
        for _i in 0..instruction[0]{
            temp_stack.push_back(stacks[instruction[1]-1].pop_back().unwrap());
        }
        for (i, char) in temp_stack.iter().enumerate().rev(){
            stacks[instruction[2]-1].push_back(*char);
        }
    }

    let mut out: Vec<char> = vec![];

    for stack in stacks{
        out.push(stack[stack.len()-1]);
    }

    String::from_iter(out).parse::<i32>().unwrap()
}
