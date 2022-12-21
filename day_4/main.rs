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

#[derive(Debug)]
struct Bounds{
    lower: i32,
    upper: i32
}

fn process_string(line: String) -> Vec<Bounds>{
    let mut boundaries : Vec<i32> = vec!();
    let mut buf : String = String::new();
    for c in line.chars(){
        if c.eq(&'-') || c.eq(&','){
            boundaries.push(buf.parse().unwrap());
            buf = String::new();
        }else{
            buf.push(c);
        }
    }
    boundaries.push(buf.parse().unwrap());
    vec![Bounds{lower: boundaries[0], upper: boundaries[1]}, Bounds{lower: boundaries[2], upper: boundaries[3]}]
}

fn part_1(){
    let lines = lines_from_file("input.txt");

    let mut total = 0;

    for line in lines{
        let bounds = process_string(line);
        if bounds[0].lower >= bounds[1].lower{
            if bounds[0].upper <= bounds[1].upper{
                total += 1;
                continue;
            }
        }
        if bounds[0].lower <= bounds[1].lower{
            if bounds[0].upper >= bounds[1].upper{
                total += 1;
                continue;
            }
        }
    }
    println!("{}", total);
}

fn part_2(){
    let lines = lines_from_file("input.txt");

    let mut total = 0;

    for line in lines{
        let bounds = process_string(line);
        if bounds[0].lower > bounds[1].upper{
            continue;
        }
        if bounds[0].upper < bounds[1].lower{
            continue;
        }
        total += 1;
    }
    println!("{}", total);
}

fn main(){
    part_1(); 
    part_2();
}