use crate::return_sol;
use crate::utils;

pub fn part1(lines: &Vec<String>) -> utils::Solution {
    let mut tot = 0;
    for line in lines {
        let game = utils::parse_game_string(line);
        if game[1] <= 12 && game[2] <= 13 && game[3] <= 14 {
            tot += game[0];
        }
    }
    return_sol!(tot)
}
pub fn part2(lines: &Vec<String>) -> utils::Solution {
    let mut tot = 0;
    for line in lines {
        let game = utils::parse_game_string(line);
        let power = game[1] * game[2] * game[3];
        tot += power;
    }
    return_sol!(tot)
}
