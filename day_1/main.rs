use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    let mut out : Vec<String> = buf.lines()
        .map(|line| line.expect("error"))
        .collect();
    out.push("".to_string());
    out
}

fn part_1(){
    let lines = lines_from_file("input.txt");

    let mut highest = 0;

    let mut current = 0;

    for line in lines
    {
        if line.eq("")
        {
            if current > highest
            {
                highest = current;
            }
            current = 0;
        }
        else
        {
            match line.parse::<i32>()
            {
                Ok(n) => current += n,
                Err(e) => println!("Error"),
            }
        }
    }

    println!("{}", highest);
}

fn part_2(){
    let lines = lines_from_file("input.txt");

    let mut highest = vec![0, 0, 0];

    let mut current = 0;

    for line in lines
    {
        if line.eq("")
        {
            if current > highest[0]
            {
                if current > highest[1]
                {
                    if current > highest[2]
                    {
                        highest[0] = highest[1];
                        highest[1] = highest[2];
                        highest[2] = current;
                    }
                    else
                    {
                        highest[0] = highest[1];
                        highest[1] = current;
                    }
                }
                else
                {
                    highest[0] = current;
                }
            }
            current = 0;
        }
        else
        {
            match line.parse::<i32>()
            {
                Ok(n) => current += n,
                Err(e) => println!("Error"),
            }
        }
    }
    highest[0] = highest[0]+highest[1]+highest[2];
    println!("{:?}", highest[0]);
}

fn main() {
    part_1();
    part_2();
}