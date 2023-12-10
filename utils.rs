use std::{
    fmt::Display,
    fmt::Formatter,
    fmt::Result,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

pub enum Solution {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),
    Str(String),
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Solution::I8(x) => x.fmt(f),
            Solution::I16(x) => x.fmt(f),
            Solution::I32(x) => x.fmt(f),
            Solution::I64(x) => x.fmt(f),
            Solution::I128(x) => x.fmt(f),
            Solution::Isize(x) => x.fmt(f),
            Solution::U8(x) => x.fmt(f),
            Solution::U16(x) => x.fmt(f),
            Solution::U32(x) => x.fmt(f),
            Solution::U64(x) => x.fmt(f),
            Solution::U128(x) => x.fmt(f),
            Solution::Usize(x) => x.fmt(f),
            Solution::Str(x) => x.fmt(f),
        }
    }
}

#[macro_export]
macro_rules! return_sol {
    ($value:expr) => {
        utils::Solution::from($value)
    };
}

macro_rules! impl_from {
    ($type_:ident, $kind_:ident) => {
        impl From<$type_> for Solution {
            fn from(sol: $type_) -> Self {
                Self::$kind_(sol)
            }
        }
    };
}

impl From<&str> for Solution {
    fn from(sol: &str) -> Self {
        Self::Str(sol.to_owned())
    }
}

impl_from!(i8, I8);
impl_from!(i16, I16);
impl_from!(i32, I32);
impl_from!(i64, I64);
impl_from!(i128, I128);
impl_from!(isize, Isize);
impl_from!(u8, U8);
impl_from!(u16, U16);
impl_from!(u32, U32);
impl_from!(u64, U64);
impl_from!(u128, U128);
impl_from!(usize, Usize);

pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    let out: Vec<String> = buf.lines().map(|line| line.expect("error")).collect();
    out
}

// Day 2 --------------------------------------------------------------

pub fn parse_day02_string(line: &String) -> Vec<u32> {
    let mut out: Vec<u32> = vec![0, 0, 0, 0];
    let mut split_line = line.split(":");
    out[0] = split_line
        .next()
        .unwrap()
        .split(" ")
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let sets = split_line.next().unwrap().split(";");
    for set in sets {
        let colours = set.split(",");
        for colour in colours {
            let val = colour.split(" ").nth(1).unwrap().parse::<u32>().unwrap();
            match colour.split(" ").last().unwrap() {
                "red" => {
                    if val > out[1] {
                        out[1] = val
                    }
                }
                "green" => {
                    if val > out[2] {
                        out[2] = val
                    }
                }
                "blue" => {
                    if val > out[3] {
                        out[3] = val
                    }
                }
                _ => println!("hit other colour?"),
            }
        }
    }
    out
}

// Day 3 --------------------------------------------------------------

pub struct Number {
    x: (usize, usize),
    y: usize,
    value: u32,
}

impl Number {
    pub fn new() -> Number {
        Number {
            x: (0, 0),
            y: 0,
            value: 0,
        }
    }

    pub fn next_to_solution(&self, symbol: &Symbol) -> bool {
        if symbol.x + 1 >= self.x.0 && symbol.x <= self.x.1 {
            if symbol.y + 1 >= self.y && symbol.y - 1 <= self.y {
                return true;
            }
        }
        false
    }

    pub fn get_value(&self) -> u32 {
        self.value
    }
}

pub struct Symbol {
    x: usize,
    y: usize,
    value: u8,
}

impl Symbol {
    pub fn get_value(&self) -> u8 {
        self.value
    }
}

pub fn parse_day03_string(lines: &Vec<String>) -> (Vec<Number>, Vec<Symbol>) {
    let mut nums: Vec<Number> = vec![];
    let mut symbols: Vec<Symbol> = vec![];
    for (j, line) in lines.iter().enumerate() {
        let mut in_number = false;
        let mut cur_number: (usize, usize) = (0, 0);
        line.as_bytes()
            .iter()
            .enumerate()
            .for_each(|(i, c)| match c {
                48..=57 => {
                    if !in_number {
                        in_number = true;
                        cur_number.0 = i;
                    }
                }
                _ => {
                    if in_number {
                        cur_number.1 = i;
                        nums.push(Number {
                            x: cur_number,
                            y: j,
                            value: line[cur_number.0..cur_number.1].parse::<u32>().unwrap(),
                        });
                        in_number = false;
                        cur_number = (0, 0);
                    }
                    if *c != '.' as u8 {
                        symbols.push(Symbol {
                            x: i,
                            y: j,
                            value: *c,
                        });
                    }
                }
            });
        if in_number {
            cur_number.1 = line.len() - 1;
            nums.push(Number {
                x: cur_number,
                y: j,
                value: line[cur_number.0..=cur_number.1].parse::<u32>().unwrap(),
            });
        }
    }
    (nums, symbols)
}
