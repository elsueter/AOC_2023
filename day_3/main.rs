use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|line| line.expect("error"))
        .collect()
}

fn part_1(){
    let lines = lines_from_file("input.txt");

    let mut total = 0;

    for line in lines
    {
        let first = &line[..line.chars().count()/2];
        let second = &line[line.chars().count()/2..];
        let mut seen_values = [0; 52];
        for c in first.chars(){
            let mut value = c as usize;
            if value > 96 {
                value -= 97;
            }else{
                value -= 39;
            }

            seen_values[value] += 1;
        }
        for c in second.chars(){
            let mut value = c as usize;
            if value > 96 {
                value -= 97;
            }else{
                value -= 39;
            }

            if seen_values[value] > 0{
                total += value + 1;
                break;
            }
        }
    }
    println!("{:?}", total);
}

fn part_2(){
    let lines = lines_from_file("input.txt");

    let mut seen_values = [0; 52];
    let mut total = 0;
    let mut mode = 0;

    for line in lines
    {
        
        if mode % 3 == 0{
            for (i, value) in seen_values.iter().enumerate(){
                if *value == 3{
                    total += i + 1;
                }
            }
            seen_values = [0; 52];
        }
        for c in line.chars(){
            let mut value = c as usize;
            if value > 96 {
                value -= 97;
            }else{
                value -= 39;
            }

            if seen_values[value] == 0 + mode % 3{
                seen_values[value] += 1;
            }
        }
        mode += 1;
    }
    for (i, value) in seen_values.iter().enumerate(){
        if *value == 3{
            total += i + 1;
        }
    }
    
    println!("{:?}", total);
}

fn main(){
    part_1();
    part_2();
}