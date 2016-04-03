#[macro_use] extern crate lazy_static;
extern crate regex;

use std::borrow::Borrow;
use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use regex::Regex;

#[derive(Debug)]
struct Things {
    name: String,
    data: HashMap<String, u8>,
}

impl Things {
    fn new(name: &str) -> Self {
        Things { name: name.to_string(), data: HashMap::new() }
    }

    fn new_from_line(line: &str) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([a-z]+): (\d+)").unwrap();
        }

        line.find(':').map(|pos| {
            let (name, things) = line.split_at(pos);

            let mut result = Self::new(name);

            for capture in RE.captures_iter(things) {
                result.set(
                    capture.at(1).unwrap(),
                    capture.at(2).unwrap().parse().unwrap(),
                );
            }
            result
        })
    }

    fn set(&mut self, key: &str, value: u8) {
        self.data.insert(key.to_string(), value);
    }

    fn _matches<F: Fn(&str, &u8, &u8) -> bool>(&self, expectations: &Self, cmp: F) -> bool {
        self.data.iter().all(|(key, value)| {
            expectations.data.get(key).map_or(false, |expected_value| cmp(key.borrow(), value, expected_value))
        })
    }

    fn matches(&self, expectations: &Self) -> bool {
        self._matches(expectations, |_, value, expected_value| value == expected_value)
    }

    fn adjusted_matches(&self, expectations: &Self) -> bool {
        self._matches(expectations, |key, value, expected_value| {
            match key {
                "cats" | "trees" => value > expected_value,
                "pomeranians" | "goldfish" => value < expected_value,
                _ => value == expected_value,
            }
        })
    }
}

fn print_aunt_sue_name_found(which: &str, name_found: Option<String>) {
    if let Some(name) = name_found {
        println!("{} Aunt Sue found: {}", which, name);
    }
    else {
        println!("{} Aunt Sue not found :(", which);
    }
}

fn main() {
    let stdin = io::stdin();

    let mut expectations = Things::new("Expectations");
    expectations.set("children", 3);
    expectations.set("cats", 7);
    expectations.set("samoyeds", 2);
    expectations.set("pomeranians", 3);
    expectations.set("akitas", 0);
    expectations.set("vizslas", 0);
    expectations.set("goldfish", 5);
    expectations.set("trees", 3);
    expectations.set("cars", 2);
    expectations.set("perfumes", 1);

    let aunt_sues = stdin.lock().lines()
        .filter_map(|l| l.ok())
        .filter_map(|ref line| Things::new_from_line(line));

    let mut unadjusted_aunt_sue: Option<String> = None;
    let mut real_aunt_sue: Option<String> = None;

    for aunt_sue in aunt_sues {
        if !unadjusted_aunt_sue.is_some() && aunt_sue.matches(&expectations) {
            unadjusted_aunt_sue = Some(aunt_sue.name.clone());
        }

        if !real_aunt_sue.is_some() && aunt_sue.adjusted_matches(&expectations) {
            real_aunt_sue = Some(aunt_sue.name.clone());
        }

        if unadjusted_aunt_sue.is_some() && real_aunt_sue.is_some() {
            break;
        }
    }

    print_aunt_sue_name_found("Unadjusted", unadjusted_aunt_sue);
    print_aunt_sue_name_found("Real", real_aunt_sue);
}
