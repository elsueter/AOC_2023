#![allow(dead_code)]
#![allow(unused_variables)]

use std::env;
use std::time::Instant;

use days::{
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25,
};

mod utils;

macro_rules! create_function {
    ($day:ident) => {
        fn $day(data_path: &Vec<String>) -> Vec<i32> {
            let now = Instant::now();
            let result = vec![
                generate_body!($day, data_path, part_1),
                generate_body!($day, data_path, part_2),
            ];
            println!("Time taken: {}", now.elapsed().as_micros());
            result
        }
    };
}

macro_rules! generate_body {
    ($day:ident, $param:ident, $part:ident) => {
        $day::$part($param)
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];

    let mode = if &args.len() <= &2 {
        &empty_string
    } else {
        &args[2]
    };

    let data_string = day.as_str().to_owned() + "/" + mode.as_str() + ".txt";
    let lines = utils::lines_from_file(data_string);

    println!("Running day {} with {}.txt", day, mode);

    println!("{:?}", result);
}
