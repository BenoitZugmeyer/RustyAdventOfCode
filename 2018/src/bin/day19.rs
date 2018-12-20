use lazy_static::lazy_static;
use regex::Regex;
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
            line.instruction.apply(line.parameters, &mut registers);
            registers[ip] += 1;
        }

        println!("Part 1: {}", registers[0]);
    }

    // Part 2
    {
        let mut registers = vec![1, 0, 0, 0, 0, 0];
        while let Some(line) = program.get(registers[ip]) {
            line.instruction.apply(line.parameters, &mut registers);
            if registers[ip] == 0 {
                break;
            }
            registers[ip] += 1;
        }

        // The program computes the sum of divisors of a large number (stored in register 4 at this
        // point), in O(n²) complexity.
        // Compute the same thing with a O(sqrt(n)) complexity.
        let target = registers[4];
        let mut result = target + 1;  // any number can be divised by 1 and itself
        let mut divisor = 2;
        while divisor * divisor <= target {
            if target % divisor == 0 {
                result += divisor;
                if divisor * divisor != target {
                    result += target / divisor;
                }
            }
            divisor += 1;
        }

        println!("Part 2: {}", result);
    }
}
