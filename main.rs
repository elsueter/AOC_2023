use std::env;
use std::time::Instant;

mod day_1;
mod day_2;

mod utils;

fn main() {
    let empty_string = String::from("");

    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let day = &args[2];

    let mode = if &args.len() <= &3 {
        &empty_string
    } else {
        &args[3]
    };

    match query.as_str() {
        "day" => {
            println!("{0} {1}", query, day);
            match day.as_str() {
                "1" => {
                    let lines = match mode.as_str() {
                        "test" => {
                            let temp_str = "day_".to_owned() + day.as_str() + "/test_input.txt";
                            utils::lines_from_file(temp_str)
                        }
                        _ => {
                            let temp_str = "day_".to_owned() + day.as_str() + "/input.txt";
                            utils::lines_from_file(temp_str)
                        }
                    };
                    let now = Instant::now();

                    let part_1 = day_1::part_1(&lines);
                    let part_2 = day_1::part_2(&lines);

                    println!("Time taken: {}", now.elapsed().as_micros());
                    println!("Part 1: {}, Part 2: {}", part_1, part_2);
                }
                "2" => {
                    let lines = match mode.as_str() {
                        "test" => utils::lines_from_file("day_2/test_input.txt"),
                        _ => utils::lines_from_file("day_2/input.txt"),
                    };
                    let now = Instant::now();

                    let part_1 = day_2::part_1(&lines);
                    let part_2 = day_2::part_2(&lines);

                    println!("Time taken: {}", now.elapsed().as_micros());
                    println!("Part 1: {}, Part 2: {}", part_1, part_2);
                }
                _ => {}
            }
        }
        _ => {}
    }
}
