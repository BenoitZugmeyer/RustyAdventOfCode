extern crate regex;
extern crate itertools;
use itertools::Itertools;
use itertools::MinMaxResult;
use std::fmt;
use std::io::stdin;
use std::io::BufRead;
use std::str::FromStr;
use regex::Regex;
use std::collections::BTreeMap;

fn parse<T: FromStr>(s: Option<&str>) -> T
    where T::Err: fmt::Display
{
    let string = s.expect("None passed to parse()");
    match string.parse() {
        Ok(n) => n,
        Err(e) => panic!("{} can't be parsed: {}", string, e),
    }
}

#[derive(Ord, Eq, PartialOrd, PartialEq, Debug, Copy, Clone)]
enum Holder {
    Bot(u32),
    Output(u32),
}

#[derive(Debug)]
enum HolderParseError {
    InvalidName,
    ParseIntError(std::num::ParseIntError),
}

impl From<std::num::ParseIntError> for HolderParseError {
    fn from(err: std::num::ParseIntError) -> HolderParseError {
        HolderParseError::ParseIntError(err)
    }
}

impl fmt::Display for HolderParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            HolderParseError::InvalidName => write!(f, "Invalid name"),
            HolderParseError::ParseIntError(ref e) => write!(f, "Parse int error: {}", e),
        }
    }
}

impl FromStr for Holder {
    type Err = HolderParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("bot ") {
            Ok(Holder::Bot(s[4..].parse()?))
        } else if s.starts_with("output ") {
            Ok(Holder::Output(s[7..].parse()?))
        } else {
            Err(HolderParseError::InvalidName)
        }
    }
}

#[derive(Debug)]
struct Transfer {
    from: Holder,
    high: Holder,
    low: Holder,
}

fn get_value(values: &mut BTreeMap<Holder, Vec<u32>>, holder: Holder) -> &mut Vec<u32> {
    values.entry(holder).or_insert_with(Vec::new)
}

fn pop_min_max(v: &mut Vec<u32>) -> Option<(u32, u32)> {
    if let MinMaxResult::MinMax(&min, &max) = v.iter().minmax() {
        v.retain(|v| v != &min && v != &max);
        Some((min, max))
    } else {
        None
    }
}

fn main() {
    let re_value = Regex::new(r"^value (\d+) goes to (.* \d+)$").unwrap();
    let re_transfer = Regex::new(r"^(.* \d+) gives low to (.* \d+) and high to (.* \d+)$").unwrap();

    let mut values: BTreeMap<Holder, Vec<u32>> = BTreeMap::new();
    let mut transfers: Vec<Transfer> = Vec::new();

    let stdin = stdin();
    for line in stdin.lock().lines().filter_map(|l| l.ok()) {
        if let Some(caps) = re_value.captures(&line) {
            get_value(&mut values, parse(caps.at(2))).push(parse(caps.at(1)));
        } else if let Some(caps) = re_transfer.captures(&line) {
            transfers.push(Transfer {
                from: parse(caps.at(1)),
                low: parse(caps.at(2)),
                high: parse(caps.at(3)),
            })
        } else {
            panic!("Invalid line {}", line);
        }
    }

    let mut responsible = None;
    loop {
        let mut had_transfer = false;
        for transfer in &transfers {
            if let Some((min, max)) = pop_min_max(get_value(&mut values, transfer.from)) {
                if min == 17 && max == 61 {
                    responsible = Some(transfer.from);
                }
                get_value(&mut values, transfer.low).push(min);
                get_value(&mut values, transfer.high).push(max);
                had_transfer = true;
            }
        }
        if !had_transfer {
            break;
        }
    }

    let mult = *get_value(&mut values, Holder::Output(0)).first().unwrap() *
               *get_value(&mut values, Holder::Output(1)).first().unwrap() *
               *get_value(&mut values, Holder::Output(2)).first().unwrap();
    println!("Part 1: {:?}", responsible);
    println!("Part 2: {:?}", mult);
}
