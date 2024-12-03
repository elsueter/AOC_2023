use crate::return_sol;
use crate::utils;

fn in_range(last: i32, curr: i32) -> bool {
    if last == curr || last - curr > 3 || last - curr < -3 {
        return false;
    }
    true
}

fn is_safe(last: i32, curr: i32, up: bool) -> bool {
    if (curr > last) == up {
        return false;
    }
    return in_range(last, curr);
}

pub fn part1(lines: &Vec<String>) -> utils::Solution {
    let mut tot = 0;
    for line in lines {
        let mut chars = line.split(" ");
        let mut last = chars.next().unwrap().parse::<i32>().unwrap();
        let mut curr = chars.next().unwrap().parse::<i32>().unwrap();
        let up = last > curr;
        let mut flag = false;
        if !in_range(last, curr) {
            flag = true;
        }
        last = curr;
        for c in chars {
            curr = c.parse::<i32>().unwrap();
            if !is_safe(last, curr, up) {
                flag = true;
                break;
            }
            last = curr;
        }
        if !flag {
            tot += 1;
        }
    }

    return_sol!(tot)
}

pub fn part2(lines: &Vec<String>) -> utils::Solution {
    let mut tot = 0;
    for line in lines {
        let mut chars = line.split(" ");
        let mut last = chars.next().unwrap().parse::<i32>().unwrap();
        let mut curr = chars.next().unwrap().parse::<i32>().unwrap();
        let up = last > curr;
        let mut flag = 0;
        if !in_range(last, curr) {
            flag += 1;
        }
        last = curr;
        for (i, c) in chars.enumerate() {
            curr = c.parse::<i32>().unwrap();
            if !is_safe(last, curr, up) {
                flag += 1;
            }
            last = curr;
        }
        if flag < 2 {
            tot += 1;
        }
    }

    return_sol!(tot)
}
