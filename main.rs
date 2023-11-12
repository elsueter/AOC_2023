#![allow(dead_code)]
#![allow(unused_variables)]

use std::env;
use std::time::Instant;

mod days;
use days::{
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25,
};

mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide the day(s) to run as a command-line argument.");
    }

    let mut start = 1;
    let mut input = "input.txt";

    if args[1] == "test" {
        start = 2;
        input = "test.txt";
        println!("\n__TEST__MODE__");
    }

    let days: Vec<u8> = args[start..]
        .iter()
        .map(|x| {
            x.parse()
                .unwrap_or_else(|v| panic!("Not a valid day: {}", v))
        })
        .collect();

    let mut runtime = 0.0;

    for day in days {
        let (p1, p2, day_str) = get_day_solver(day);

        let lines = utils::lines_from_file("input/day".to_owned() + day_str.as_str() + "_" + input);
        let time = Instant::now();
        let r1 = p1(&lines);
        let r2 = p2(&lines);
        let elapsed = time.elapsed().as_nanos() as f64 / 1_000_000.0;

        println!("\n=== Day {:02} ===", day);
        println!("  · Part 1: {}", r1);
        println!("  · Part 2: {}", r2);
        println!("  · Elapsed: {:.4} ms", elapsed);

        runtime += elapsed;
    }

    println!("\nTotal runtime: {:.4} ms\n", runtime);
}

fn get_day_solver(
    day: u8,
) -> (
    fn(&Vec<String>) -> utils::Solution,
    fn(&Vec<String>) -> utils::Solution,
    String,
) {
    match day {
        1 => (day01::part1, day01::part2, "01".to_string()),
        2 => (day02::part1, day02::part2, "02".to_string()),
        3 => (day03::part1, day03::part2, "03".to_string()),
        4 => (day04::part1, day04::part2, "04".to_string()),
        5 => (day05::part1, day05::part2, "05".to_string()),
        6 => (day06::part1, day06::part2, "06".to_string()),
        7 => (day07::part1, day07::part2, "07".to_string()),
        8 => (day08::part1, day08::part2, "08".to_string()),
        9 => (day09::part1, day09::part2, "09".to_string()),
        10 => (day10::part1, day10::part2, "10".to_string()),
        11 => (day11::part1, day11::part2, "11".to_string()),
        12 => (day12::part1, day12::part2, "12".to_string()),
        13 => (day13::part1, day13::part2, "13".to_string()),
        14 => (day14::part1, day14::part2, "14".to_string()),
        15 => (day15::part1, day15::part2, "15".to_string()),
        16 => (day16::part1, day16::part2, "16".to_string()),
        17 => (day17::part1, day17::part2, "17".to_string()),
        18 => (day18::part1, day18::part2, "18".to_string()),
        19 => (day19::part1, day19::part2, "19".to_string()),
        20 => (day20::part1, day20::part2, "20".to_string()),
        21 => (day21::part1, day21::part2, "21".to_string()),
        22 => (day22::part1, day22::part2, "22".to_string()),
        23 => (day23::part1, day23::part2, "23".to_string()),
        24 => (day24::part1, day24::part2, "24".to_string()),
        25 => (day25::part1, day25::part2, "25".to_string()),
        _ => unimplemented!(),
    }
}
