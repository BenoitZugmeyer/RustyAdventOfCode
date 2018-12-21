use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::io::{stdin, BufRead};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instructions {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

impl Instructions {
    fn apply(self, (a, b, c): (usize, usize, usize), registers: &mut [usize]) {
        match self {
            Instructions::Addr => registers[c] = registers[a] + registers[b],
            Instructions::Addi => registers[c] = registers[a] + b,
            Instructions::Mulr => registers[c] = registers[a] * registers[b],
            Instructions::Muli => registers[c] = registers[a] * b,
            Instructions::Banr => registers[c] = registers[a] & registers[b],
            Instructions::Bani => registers[c] = registers[a] & b,
            Instructions::Borr => registers[c] = registers[a] | registers[b],
            Instructions::Bori => registers[c] = registers[a] | b,
            Instructions::Setr => registers[c] = registers[a],
            Instructions::Seti => registers[c] = a,
            Instructions::Gtir => registers[c] = if a > registers[b] { 1 } else { 0 },
            Instructions::Gtri => registers[c] = if registers[a] > b { 1 } else { 0 },
            Instructions::Gtrr => registers[c] = if registers[a] > registers[b] { 1 } else { 0 },
            Instructions::Eqir => registers[c] = if a == registers[b] { 1 } else { 0 },
            Instructions::Eqri => registers[c] = if registers[a] == b { 1 } else { 0 },
            Instructions::Eqrr => registers[c] = if registers[a] == registers[b] { 1 } else { 0 },
        }
    }
}

#[derive(Debug)]
struct ProgramLine {
    instruction: Instructions,
    parameters: (usize, usize, usize),
}

impl std::str::FromStr for ProgramLine {
    type Err = ();
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^([a-z]{4}) (\d+) (\d+) (\d+)").unwrap();
        }
        if let Some(caps) = RE.captures(&line) {
            Ok(Self {
                instruction: match caps.get(1).unwrap().as_str() {
                    "addr" => Instructions::Addr,
                    "addi" => Instructions::Addi,
                    "mulr" => Instructions::Mulr,
                    "muli" => Instructions::Muli,
                    "banr" => Instructions::Banr,
                    "bani" => Instructions::Bani,
                    "borr" => Instructions::Borr,
                    "bori" => Instructions::Bori,
                    "setr" => Instructions::Setr,
                    "seti" => Instructions::Seti,
                    "gtir" => Instructions::Gtir,
                    "gtri" => Instructions::Gtri,
                    "gtrr" => Instructions::Gtrr,
                    "eqir" => Instructions::Eqir,
                    "eqri" => Instructions::Eqri,
                    "eqrr" => Instructions::Eqrr,
                    _ => return Err(()),
                },
                parameters: (
                    caps.get(2).unwrap().as_str().parse().unwrap(),
                    caps.get(3).unwrap().as_str().parse().unwrap(),
                    caps.get(4).unwrap().as_str().parse().unwrap(),
                ),
            })
        } else {
            Err(())
        }
    }
}

fn main() {
    // let mut test_program = Vec::new();
    let stdin = stdin();
    let mut lines = stdin.lock().lines().filter_map(|l| l.ok());

    let ip: String = lines.next().unwrap().chars().skip(4).collect();
    let ip: usize = ip.parse().unwrap();

    let program: Vec<ProgramLine> = lines.filter_map(|l| l.parse().ok()).collect();

    // Part 1
    {
        let mut registers = vec![0; 6];
        while let Some(line) = program.get(registers[ip]) {
            if line.instruction == Instructions::Eqrr
                && line.parameters.0 == 4
                && line.parameters.1 == 0
            {
                break;
            }
            line.instruction.apply(line.parameters, &mut registers);
            registers[ip] += 1;
        }

        println!("Part 1: {}", registers[4]);
    }

    // Part 2
    {
        let mut set = HashSet::new();
        let mut result = 0;

        // This is the rust implementation of the program, to be a bit faster.
        let mut r4 = 0;
        loop {
            let mut r3 = r4 | 0x10000;
            r4 = 707_129;
            loop {
                r4 = (((r4 + (r3 & 0xff)) & 0xff_ffff) * 65899) & 0xff_ffff;
                if r3 < 0x100 {
                    break;
                }
                r3 /= 0x100;
            }

            if set.insert(r4) {
                result = r4;
            } else {
                break;
            }
            // if r4 == r0 {
            //     break;
            // }
        }

        // We could run the program until we find the result too, but it's longer:
        // let mut registers = vec![0; 6];
        // while let Some(line) = program.get(registers[ip]) {
        //     if line.instruction == Instructions::Eqrr
        //         && line.parameters.0 == 4
        //         && line.parameters.1 == 0
        //     {
        //         if set.insert(registers[4]) {
        //             result = registers[4];
        //         } else {
        //             break;
        //         }
        //     }
        //     line.instruction.apply(line.parameters, &mut registers);
        //     registers[ip] += 1;
        // }

        println!("Part 2: {}", result);
    }
}
