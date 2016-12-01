#[macro_use] extern crate lazy_static;
extern crate regex;

use std::error::Error;
use std::io;
use std::io::BufRead;
use std::convert::From;
use std::collections::HashMap;
use regex::Regex;

type Register = char;
type Offset = i32;

#[derive(Debug)]
enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(Offset),
    Jie(Register, Offset),
    Jio(Register, Offset),
}

impl Instruction {
    fn parse(s: &str) -> Result<Self, Box<Error>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^([a-z]{3}) ([a-z]|[+-]\d+)(?:, ([+-]\d+))?$").unwrap();
        }

        macro_rules! parse_register {
            ($str: expr) => (
                try!(try!($str.ok_or("No register specified")).chars().next().ok_or("Invalid register name"))
            )
        }

        macro_rules! parse_offset {
            ($str: expr) => (
                try!(try!($str.ok_or("No offset specified")).parse::<Offset>())
            )
        }

        if let Some(caps) = RE.captures(&s) {
            match try!(caps.at(1).ok_or("No first group")) {
                "hlf" => Ok(Instruction::Hlf(parse_register!(caps.at(2)))),
                "tpl" => Ok(Instruction::Tpl(parse_register!(caps.at(2)))),
                "inc" => Ok(Instruction::Inc(parse_register!(caps.at(2)))),
                "jmp" => Ok(Instruction::Jmp(parse_offset!(caps.at(2)))),
                "jie" => Ok(Instruction::Jie(parse_register!(caps.at(2)), parse_offset!(caps.at(3)))),
                "jio" => Ok(Instruction::Jio(parse_register!(caps.at(2)), parse_offset!(caps.at(3)))),
                name => Err(From::from(format!("Unknown instruction {}", name))),
            }
        }
        else {
            Err(From::from("Does not match regex"))
        }
    }
}

#[derive(Default)]
struct Computer {
    registers: HashMap<char, u32>,
}

impl Computer {
    fn new() -> Self {
        Computer { registers: HashMap::new() }
    }

    fn execute(&mut self, instructions: &[Instruction]) {
        let mut index: i32 = 0;

        macro_rules! register {
            ($name: expr) => ( *self.registers.entry($name).or_insert(0) )
        }

        while index >= 0 && index < instructions.len() as i32 {
            match instructions[index as usize] {
                Instruction::Hlf(r) => {
                    register!(r) /= 2;
                    index += 1;
                },
                Instruction::Tpl(r) => {
                    register!(r) *= 3;
                    index += 1;
                },
                Instruction::Inc(r) => {
                    register!(r) += 1;
                    index += 1;
                },
                Instruction::Jmp(offset) => {
                    index += offset;
                },
                Instruction::Jie(r, offset) => {
                    index += if register!(r) % 2 == 0 { offset } else { 1 }
                },
                Instruction::Jio(r, offset) => {
                    index += if register!(r) == 1 { offset } else { 1 }
                },
            }
        }

    }
}

#[test]
fn example() {
    let instructions = vec![
        Instruction::parse("inc a").unwrap(),
        Instruction::parse("jio a, +2").unwrap(),
        Instruction::parse("tpl a").unwrap(),
        Instruction::parse("inc a").unwrap(),
    ];

    let mut computer = Computer::new();
    computer.execute(&instructions);
    assert_eq!(computer.registers.get(&'a'), Some(&2));
}

fn main() {
    let stdin = io::stdin();

    let instructions = stdin.lock().lines()
        .filter_map(|l| l.ok())
        .filter_map(|line| {
            match Instruction::parse(&line) {
                Err(error) => {
                    println!("Skipping line '{}': {}", line, error.to_string());
                    None
                },
                Ok(result) => Some(result),
            }
        })
        .collect::<Vec<_>>();

    let mut computer = Computer::new();
    computer.execute(&instructions);

    println!("Register 'b' after executing instructions: {:?}", computer.registers.get(&'b'));

    computer.registers.clear();
    computer.registers.insert('a', 1);
    computer.execute(&instructions);

    println!("Register 'b' after executing instructions with 'a' initialized to 1: {:?}", computer.registers.get(&'b'));
}

