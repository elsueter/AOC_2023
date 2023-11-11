mkdir day_1
echo '
pub fn part_1(lines:&vec<string>) -> i32{0}
pub fn part_2(lines:&vec<string>) -> i32{0}
' >day_1/mod.rs
mkdir day_2
echo '
pub fn part_1(lines:&vec<string>) -> i32{0}
pub fn part_2(lines:&vec<string>) -> i32{0}
' >day_2/mod.rs
mkdir day_3
echo '
pub fn part_1(lines:&vec<string>) -> i32{0}
pub fn part_2(lines:&vec<string>) -> i32{0}
' >day_3/mod.rs
mkdir day_4
echo '
pub fn part_1(lines:&vec<string>) -> i32{0}
pub fn part_2(lines:&vec<string>) -> i32{0}
' >day_4/mod.rs
mkdir day_5
echo '
pub fn part_1(lines:&vec<string>) -> i32{0}
pub fn part_2(lines:&vec<string>) -> i32{0}
' >day_5/mod.rs
mkdir day_6
echo '
pub fn part_1(lines:&vec<string>) -> i32{0}
pub fn part_2(lines:&vec<string>) -> i32{0}
' >day_6/mod.rs
mkdir day_7
echo '
pub fn part_1(lines:&vec<string>) -> i32{0}
pub fn part_2(lines:&vec<string>) -> i32{0}
' >day_7/mod.rs
mkdir day_8
echo '
pub fn part_1(lines:&vec<string>) -> i32{0}
pub fn part_2(lines:&vec<string>) -> i32{0}
' >day_8/mod.rs
mkdir day_9
echo '
pub fn part_1(lines:&vec<string>) -> i32{0}
pub fn part_2(lines:&vec<string>) -> i32{0}
' >day_9/mod.rs
mkdir day_10
echo '
pub fn part_1(lines:&vec<string>) -> i32{0}
pub fn part_2(lines:&vec<string>) -> i32{0}
' >day_10/mod.rs
mkdir day_11
echo '
pub fn part_1(lines:&vec<string>) -> i32{0}
pub fn part_2(lines:&vec<string>) -> i32{0}
' >day_11/mod.rs
mkdir day_12
echo '
pub fn part_1(lines:&vec<string>) -> i32{0}
pub fn part_2(lines:&vec<string>) -> i32{0}
' >day_12/mod.rs
mkdir day_13
echo '
pub fn part_1(lines:&vec<string>) -> i32{0}
pub fn part_2(lines:&vec<string>) -> i32{0}
' >day_13/mod.rs
mkdir day_14
echo '
pub fn part_1(lines:&vec<string>) -> i32{0}
pub fn part_2(lines:&vec<string>) -> i32{0}
' >day_14/mod.rs
mkdir day_15
echo '
pub fn part_1(lines:&vec<string>) -> i32{0}
pub fn part_2(lines:&vec<string>) -> i32{0}
' >day_15/mod.rs
mkdir day_16
echo '
pub fn part_1(lines:&vec<string>) -> i32{0}
pub fn part_2(lines:&vec<string>) -> i32{0}
' >day_16/mod.rs
mkdir day_17
echo '
pub fn part_1(lines:&vec<string>) -> i32{0}
pub fn part_2(lines:&vec<string>) -> i32{0}
' >day_17/mod.rs
mkdir day_18
echo '
pub fn part_1(lines:&vec<string>) -> i32{0}
pub fn part_2(lines:&vec<string>) -> i32{0}
' >day_18/mod.rs
mkdir day_19
echo '
pub fn part_1(lines:&vec<string>) -> i32{0}
pub fn part_2(lines:&vec<string>) -> i32{0}
' >day_19/mod.rs
mkdir day_20
echo '
pub fn part_1(lines:&vec<string>) -> i32{0}
pub fn part_2(lines:&vec<string>) -> i32{0}
' >day_20/mod.rs
mkdir day_21
echo '
pub fn part_1(lines:&vec<string>) -> i32{0}
pub fn part_2(lines:&vec<string>) -> i32{0}
' >day_21/mod.rs
mkdir day_22
echo '
pub fn part_1(lines:&vec<string>) -> i32{0}
pub fn part_2(lines:&vec<string>) -> i32{0}
' >day_22/mod.rs
mkdir day_23
echo '
pub fn part_1(lines:&vec<string>) -> i32{0}
pub fn part_2(lines:&vec<string>) -> i32{0}
' >day_23/mod.rs
mkdir day_24
echo '
pub fn part_1(lines:&vec<string>) -> i32{0}
pub fn part_2(lines:&vec<string>) -> i32{0}
' >day_24/mod.rs
mkdir day_25
echo '
pub fn part_1(lines:&vec<string>) -> i32{0}
pub fn part_2(lines:&vec<string>) -> i32{0}
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    let query = &args[2];

    match query.as_str() {
        }
        _ => {}
    }
}
'
