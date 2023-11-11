//use crate::utils;
use std::collections::VecDeque;

fn split_vector(input: Vec<String>) -> Vec<Vec<String>>{
    let mut out: Vec<Vec<String>> = vec![vec![]];
    let mut mode = 0;

    out.push(vec![]);

    for line in input{
        if line.starts_with("move"){
            mode = 1;
        }
        out[mode].push(line);
    }

    out[0].pop();
    out[0].pop();

    out
}

fn parse_stacks(input: &Vec<String>) -> Vec<VecDeque<char>>{

    let num_stacks = (input[0].len()+1)/4;

    let mut out: Vec<VecDeque<char>> = vec![VecDeque::new(); num_stacks];
    
    for line in input{
        for i in 0..num_stacks{
            let cur_char = line.as_bytes()[(1+(4*i))] as char;
            if cur_char as usize != 32{
                out[i].push_front(cur_char);
            }
        }
    }

    out
}

fn parse_instructions(input: &Vec<String>) -> Vec<Vec<usize>>{
    let mut out: Vec<Vec<usize>> = vec![];
    
    for line in input{
        let mut values: Vec<usize> = vec![];
        for word in line.split(" "){
            match word.parse::<usize>(){
                Ok(x) => values.push(x),
                Err(_) => (),
            }
        }
        out.push(values);
    }

    out
}

pub fn part_1(lines: &Vec<String>) -> i32{
    let temp = split_vector(lines.clone());

    let mut stacks = parse_stacks(&temp[0]);
    let instructions = parse_instructions(&temp[1]);

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
    let temp = split_vector(lines.clone());

    let mut stacks = parse_stacks(&temp[0]);
    let instructions = parse_instructions(&temp[1]);

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
