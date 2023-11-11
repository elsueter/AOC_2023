mkdir day_1
echo '
pub fn part_1(lines:&Vec<String>) -> i32{0}
pub fn part_2(lines:&Vec<String>) -> i32{0}
' >day_1/mod.rs
mkdir day_2
echo '
pub fn part_1(lines:&Vec<String>) -> i32{0}
pub fn part_2(lines:&Vec<String>) -> i32{0}
' >day_2/mod.rs
mkdir day_3
echo '
pub fn part_1(lines:&Vec<String>) -> i32{0}
pub fn part_2(lines:&Vec<String>) -> i32{0}
' >day_3/mod.rs
mkdir day_4
echo '
pub fn part_1(lines:&Vec<String>) -> i32{0}
pub fn part_2(lines:&Vec<String>) -> i32{0}
' >day_4/mod.rs
mkdir day_5
echo '
pub fn part_1(lines:&Vec<String>) -> i32{0}
pub fn part_2(lines:&Vec<String>) -> i32{0}
' >day_5/mod.rs
mkdir day_6
echo '
pub fn part_1(lines:&Vec<String>) -> i32{0}
pub fn part_2(lines:&Vec<String>) -> i32{0}
' >day_6/mod.rs
mkdir day_7
echo '
pub fn part_1(lines:&Vec<String>) -> i32{0}
pub fn part_2(lines:&Vec<String>) -> i32{0}
' >day_7/mod.rs
mkdir day_8
echo '
pub fn part_1(lines:&Vec<String>) -> i32{0}
pub fn part_2(lines:&Vec<String>) -> i32{0}
' >day_8/mod.rs
mkdir day_9
echo '
pub fn part_1(lines:&Vec<String>) -> i32{0}
pub fn part_2(lines:&Vec<String>) -> i32{0}
' >day_9/mod.rs
mkdir day_10
echo '
pub fn part_1(lines:&Vec<String>) -> i32{0}
pub fn part_2(lines:&Vec<String>) -> i32{0}
' >day_10/mod.rs
mkdir day_11
echo '
pub fn part_1(lines:&Vec<String>) -> i32{0}
pub fn part_2(lines:&Vec<String>) -> i32{0}
' >day_11/mod.rs
mkdir day_12
echo '
pub fn part_1(lines:&Vec<String>) -> i32{0}
pub fn part_2(lines:&Vec<String>) -> i32{0}
' >day_12/mod.rs
mkdir day_13
echo '
pub fn part_1(lines:&Vec<String>) -> i32{0}
pub fn part_2(lines:&Vec<String>) -> i32{0}
' >day_13/mod.rs
mkdir day_14
echo '
pub fn part_1(lines:&Vec<String>) -> i32{0}
pub fn part_2(lines:&Vec<String>) -> i32{0}
' >day_14/mod.rs
mkdir day_15
echo '
pub fn part_1(lines:&Vec<String>) -> i32{0}
pub fn part_2(lines:&Vec<String>) -> i32{0}
' >day_15/mod.rs
mkdir day_16
echo '
pub fn part_1(lines:&Vec<String>) -> i32{0}
pub fn part_2(lines:&Vec<String>) -> i32{0}
' >day_16/mod.rs
mkdir day_17
echo '
pub fn part_1(lines:&Vec<String>) -> i32{0}
pub fn part_2(lines:&Vec<String>) -> i32{0}
' >day_17/mod.rs
mkdir day_18
echo '
pub fn part_1(lines:&Vec<String>) -> i32{0}
pub fn part_2(lines:&Vec<String>) -> i32{0}
' >day_18/mod.rs
mkdir day_19
echo '
pub fn part_1(lines:&Vec<String>) -> i32{0}
pub fn part_2(lines:&Vec<String>) -> i32{0}
' >day_19/mod.rs
mkdir day_20
echo '
pub fn part_1(lines:&Vec<String>) -> i32{0}
pub fn part_2(lines:&Vec<String>) -> i32{0}
' >day_20/mod.rs
mkdir day_21
echo '
pub fn part_1(lines:&Vec<String>) -> i32{0}
pub fn part_2(lines:&Vec<String>) -> i32{0}
' >day_21/mod.rs
mkdir day_22
echo '
pub fn part_1(lines:&Vec<String>) -> i32{0}
pub fn part_2(lines:&Vec<String>) -> i32{0}
' >day_22/mod.rs
mkdir day_23
echo '
pub fn part_1(lines:&Vec<String>) -> i32{0}
pub fn part_2(lines:&Vec<String>) -> i32{0}
' >day_23/mod.rs
mkdir day_24
echo '
pub fn part_1(lines:&Vec<String>) -> i32{0}
pub fn part_2(lines:&Vec<String>) -> i32{0}
' >day_24/mod.rs
mkdir day_25
echo '
pub fn part_1(lines:&Vec<String>) -> i32{0}
pub fn part_2(lines:&Vec<String>) -> i32{0}
' >day_25/mod.rs

echo '
use std::env;
use std::time::Instant;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;

mod utils;

macro_rules! create_function {
    ($day:ident) => {
        fn $day(data_path:&Vec<String>) -> Vec<i32> {
            vec![generate_body!($day, data_path, part_1), generate_body!($day, data_path, part_2)]
        }
    };
}

macro_rules! generate_body {
    ($day:ident, $param:ident, $part:ident) => {
        $day::$part($param)
    }
}

fn main() {
    let empty_string = String::from("");

    let args: Vec<String> = env::args().collect();
    let day = &args[1];

    let mode = if &args.len() <= &2 {
        &empty_string
    } else {
        &args[2]
    };

    let mut result: Vec<i32> = vec![];

    let data_string = day.as_str().to_owned() + "/" + mode.as_str() + ".txt";
    let lines = utils::lines_from_file(data_string);

    println!("Running {} with {}.txt", day, mode);

    create_function!(day_1);
    create_function!(day_2);
    create_function!(day_3);
    create_function!(day_4);
    create_function!(day_5);
    create_function!(day_6);
    create_function!(day_7);
    create_function!(day_8);
    create_function!(day_9);
    create_function!(day_10);
    create_function!(day_11);
    create_function!(day_12);
    create_function!(day_13);
    create_function!(day_14);
    create_function!(day_15);
    create_function!(day_16);
    create_function!(day_17);
    create_function!(day_18);
    create_function!(day_19);
    create_function!(day_20);
    create_function!(day_21);
    create_function!(day_22);
    create_function!(day_23);
    create_function!(day_24);
    create_function!(day_25);
 
    let now = Instant::now();

    match day.as_str() {
        "day_1" => {
            result = day_1(&lines);
        }
        "day_2" => {
            result = day_2(&lines);
        }
        "day_3" => {
            result = day_3(&lines);
        }
        "day_4" => {
            result = day_4(&lines);
        }
        "day_5" => {
            result = day_5(&lines);
        }
        "day_6" => {
            result = day_6(&lines);
        }
        "day_7" => {
            result = day_7(&lines);
        }
        "day_8" => {
            result = day_8(&lines);
        }
        "day_9" => {
            result = day_9(&lines);
        }
        "day_10" => {
            result = day_10(&lines);
        }
        "day_11" => {
            result = day_11(&lines);
        }
        "day_12" => {
            result = day_12(&lines);
        }
        "day_13" => {
            result = day_13(&lines);
        }
        "day_14" => {
            result = day_14(&lines);
        }
        "day_15" => {
            result = day_15(&lines);
        }
        "day_16" => {
            result = day_16(&lines);
        }
        "day_17" => {
            result = day_17(&lines);
        }
        "day_18" => {
            result = day_18(&lines);
        }
        "day_19" => {
            result = day_19(&lines);
        }
        "day_20" => {
            result = day_20(&lines);
        }
        "day_21" => {
            result = day_21(&lines);
        }
        "day_22" => {
            result = day_22(&lines);
        }
        "day_23" => {
            result = day_23(&lines);
        }
        "day_24" => {
            result = day_24(&lines);
        }
        "day_25" => {
            result = day_25(&lines);
        }
        _ => {}
    }

    println!("{:?}", result);

    println!("Time taken: {}", now.elapsed().as_micros());
}
