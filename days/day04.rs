use crate::return_sol;
use crate::utils;
pub fn part1(lines: &Vec<String>) -> utils::Solution {
    let mut tot = 0;
    for line in lines {
        let mut split_line = line.split("|");
        let card_1 = split_line.next().unwrap().split(":").nth(1).unwrap();
        let card_2 = split_line.next().unwrap();

        let mut winning_numbers: Vec<u8> = vec![];

        card_1.split(" ").for_each(|num| {
            if num != "" {
                winning_numbers.push(num.parse::<u8>().unwrap());
            }
        });
        let mut score = 1;
        card_2.split(" ").for_each(|num| {
            if num != "" {
                if winning_numbers.contains(&num.parse::<u8>().unwrap()) {
                    score <<= 1;
                }
            }
        });
        tot += score >> 1;
    }
    return_sol!(tot)
}
pub fn part2(lines: &Vec<String>) -> utils::Solution {
    let mut tot = 0;
    let mut line_totals: Vec<(u32, u32)> = vec![];

    for (i, line) in lines.iter().enumerate() {
        if line_totals.len() <= i {
            line_totals.push((0, 0));
        }

        let mut split_line = line.split("|");
        let card_1 = split_line.next().unwrap().split(":").nth(1).unwrap();
        let card_2 = split_line.next().unwrap();

        let mut winning_numbers: Vec<u8> = vec![];

        card_1.split(" ").for_each(|num| {
            if num != "" {
                winning_numbers.push(num.parse::<u8>().unwrap());
            }
        });
        let mut score = 0;
        card_2.split(" ").for_each(|num| {
            if num != "" {
                if winning_numbers.contains(&num.parse::<u8>().unwrap()) {
                    score += 1;
                }
            }
        });

        line_totals[i].1 = 1 << score;

        for j in 1..=score {
            if line_totals.len() <= i + j {
                line_totals.push((0, 0));
            }

            line_totals[i + j].0 += 1 * (line_totals[i].0 + 1);
        }
    }

    for pair in line_totals {
        tot += pair.0 + 1;
    }
    return_sol!(tot)
}
