//use crate::utils;

pub fn part_1(lines: &Vec<String>) -> i32 {
    let mut highest = 0;

    let mut current = 0;

    for line in lines {
        if line.eq("") {
            if current > highest {
                highest = current;
            }
            current = 0;
        } else {
            match line.parse::<i32>() {
                Ok(n) => current += n,
                Err(_e) => println!("Error"),
            }
        }
    }

    highest
}

pub fn part_2(lines: &Vec<String>) -> i32 {
    let mut highest = vec![0, 0, 0];

    let mut current = 0;

    for line in lines {
        if line.eq("") {
            if current > highest[0] {
                if current > highest[1] {
                    if current > highest[2] {
                        highest[0] = highest[1];
                        highest[1] = highest[2];
                        highest[2] = current;
                    } else {
                        highest[0] = highest[1];
                        highest[1] = current;
                    }
                } else {
                    highest[0] = current;
                }
            }
            current = 0;
        } else {
            match line.parse::<i32>() {
                Ok(n) => current += n,
                Err(_e) => println!("Error"),
            }
        }
    }
    highest[0] = highest[0] + highest[1] + highest[2];
    highest[0]
}
