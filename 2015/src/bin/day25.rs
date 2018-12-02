extern crate regex;

use std::io;
use std::io::BufRead;
use regex::Regex;

fn code_number(row: u32, column: u32) -> u32 {
    let n = row + column - 1;
    column + (n * (n - 1)) / 2
}

fn code(number: u32) -> u32 {
    let mut result = 20_151_125u64;
    for _ in 1..number {
        result = (result * 252_533) % 33_554_393
    }
    result as u32
}

#[test]
fn code_number_test() {
    assert_eq!(code_number(4, 2), 12);
    assert_eq!(code_number(1, 5), 15);
}

#[test]
fn code_test() {
    assert_eq!(code(1), 20151125);
    assert_eq!(code(2), 31916031);
    assert_eq!(code(3), 18749137);
    assert_eq!(code(11), 77061);
}


fn main() {
    let regex = Regex::new(r"row (\d+), column (\d+)").unwrap();
    let line = io::stdin().lock().lines().next().unwrap().unwrap();
    let cap = regex.captures(&line).unwrap();
    let row: u32 = cap.at(1).unwrap().parse().unwrap();
    let column: u32 = cap.at(2).unwrap().parse().unwrap();

    let number = code_number(row, column);
    println!("Code to give to the machine: {}", code(number));
}
