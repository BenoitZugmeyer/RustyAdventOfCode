extern crate regex;
use regex::Regex;
use std::io::stdin;
use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    let stdin = stdin();
    let re = Regex::new(r"^(\w+) (inc|dec) (-?\d+) if (\w+) (.*) (-?\d+)$")
        .expect("failed to parse regex");
    let mut registers: HashMap<String, i32> = HashMap::new();
    let mut highest_value: Option<i32> = None;
    for line in stdin.lock().lines().filter_map(|l| l.ok()) {
        if let Some(captures) = re.captures(&line) {
            let condition_register = *registers.entry(captures[4].to_string()).or_insert(0);
            let condition_value = captures[6].parse().unwrap();
            let ok = match &captures[5] {
                "<" => condition_register < condition_value,
                ">" => condition_register > condition_value,
                "==" => condition_register == condition_value,
                "!=" => condition_register != condition_value,
                ">=" => condition_register >= condition_value,
                "<=" => condition_register <= condition_value,
                _ => panic!("Unknown operation {}", &captures[5]),
            };
            if ok {
                let mut register = registers.entry(captures[1].to_string()).or_insert(0);
                let mut value: i32 = captures[3].parse().unwrap();
                match &captures[2] {
                    "inc" => *register += value,
                    "dec" => *register -= value,
                    _ => panic!("Unknown action {}", &captures[2]),
                }
                highest_value = highest_value.map(|hv| hv.max(*register)).or_else(
                    || Some(*register),
                );
            }
        } else {
            panic!(format!("Failed to parse line {}", line));
        };
    }

    println!("Part 1: {}", registers.values().max().unwrap());
    println!("Part 2: {}", highest_value.unwrap());
}
