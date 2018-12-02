#[macro_use] extern crate lazy_static;
extern crate regex;

use std::io;
use std::io::BufRead;
use std::collections::BTreeMap;
use regex::Regex;

type Circuit = BTreeMap<u32, Operation>;
type Signals = BTreeMap<u32, u16>;

#[derive(Debug)]
enum Entry {
    Number(u16),
    Wire(u32),
}

#[derive(Debug)]
enum Operation {
    Identity(Entry),
    Not(Entry),
    And(Entry, Entry),
    Or(Entry, Entry),
    LShift(Entry, Entry),
    RShift(Entry, Entry),
}

macro_rules! signal {
    ($signals: expr, $entry: expr) => {
        match $entry.get_signal(&$signals) {
            Some(n) => n,
            None => return None,
        }
    }
}


impl Entry {
    fn get_signal(&self, signals: &Signals) -> Option<u16> {
        match *self {
            Entry::Number(n) => Some(n),
            Entry::Wire(ref n) => signals.get(n).cloned(),
        }
    }
}

impl Operation {
    fn get_signal(&self, s: &Signals) -> Option<u16> {
        Some(match *self {
            Operation::Identity(ref input) => signal!(s, input),
            Operation::And(ref left, ref right) => signal!(s, left) & signal!(s, right),
            Operation::Or(ref left, ref right) => signal!(s, left) | signal!(s, right),
            Operation::Not(ref input) => signal!(s, input) ^ 0xffff,
            Operation::RShift(ref left, ref right) => signal!(s, left) >> signal!(s, right),
            Operation::LShift(ref left, ref right) => signal!(s, left) << signal!(s, right),
        })
    }
}


fn parse_wire(wire: &str) -> u32 {
    wire.bytes().rev().enumerate().fold(0u32, |total, (index, ch)| {
        total + (u32::from(ch) - 96) * 26u32.pow(index as u32)
    })
}

fn parse_entry(number: &str, wire: &str) -> Entry {
    if ! number.is_empty() {
        Entry::Number(number.parse().unwrap())
    }
    else if wire.is_empty() {
        Entry::Number(0)
    }
    else {
        Entry::Wire(parse_wire(wire))
    }
}

fn parse(s: &str) -> Option<(u32, Operation)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d*)([a-z]*) ?([A-Z]*) ?(\d*)([a-z]*) -> ([a-z]+)").unwrap();
    }

    RE.captures(s).map(|ref m| {
        let left = parse_entry(m.at(1).unwrap(), m.at(2).unwrap());
        let right = parse_entry(m.at(4).unwrap(), m.at(5).unwrap());

        let operation = match m.at(3).unwrap() {
            "AND" => Operation::And(left, right),
            "OR" => Operation::Or(left, right),
            "RSHIFT" => Operation::RShift(left, right),
            "LSHIFT" => Operation::LShift(left, right),
            "NOT" => Operation::Not(right),
            "" => Operation::Identity(left),
            _ => panic!("Unknown operation {}", m.at(3).unwrap())
        };

        (parse_wire(m.at(6).unwrap()), operation)
    })
}

fn run_circuit(circuit: &Circuit, signals: &mut Signals) -> Option<u16> {
    let gate_a = parse_wire("a");
    let mut signal_found = true;

    while signal_found && !signals.contains_key(&gate_a) {
        signal_found = false;
        for (output, operation) in circuit.iter() {
            if !signals.contains_key(output) {
                let signal = operation.get_signal(&signals);
                if let Some(s) = signal {
                    signals.insert(*output, s);
                    signal_found = true;
                }
            }
        }
    }

    signals.get(&gate_a).cloned()
}

fn main() {

    let circuit = io::stdin().lock().lines()
        .filter_map(|l| l.ok())
        .filter_map(|ref line| parse(&line))
        .collect::<Circuit>();

    let mut signals = Signals::new();

    let signal_b = match run_circuit(&circuit, &mut signals) {
        None => {
            println!("Signal not found on first pass :(");
            return;
        },
        Some(s) => {
            println!("Signal on first pass: {}", s);
            s
        }
    };

    signals.clear();
    signals.insert(parse_wire("b"), signal_b);

    match run_circuit(&circuit, &mut signals) {
        None => println!("Signal not found on second pass :("),
        Some(s) => println!("Signal on second pass: {}", s),
    }
}
