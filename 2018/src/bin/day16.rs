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

static INSTRUCTIONS: [Instructions; 16] = [
    Instructions::Addr,
    Instructions::Addi,
    Instructions::Mulr,
    Instructions::Muli,
    Instructions::Banr,
    Instructions::Bani,
    Instructions::Borr,
    Instructions::Bori,
    Instructions::Setr,
    Instructions::Seti,
    Instructions::Gtir,
    Instructions::Gtri,
    Instructions::Gtrr,
    Instructions::Eqir,
    Instructions::Eqri,
    Instructions::Eqrr,
];

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

    fn test_sample(self, sample: &Sample) -> bool {
        let mut registers = sample.before.clone();
        self.apply(sample.line.parameters, &mut registers);
        registers == sample.after
    }
}

#[derive(Debug)]
struct Sample {
    before: Vec<usize>,
    line: ProgramLine,
    after: Vec<usize>,
}

#[derive(Debug)]
struct ProgramLine {
    opcode: usize,
    parameters: (usize, usize, usize),
}

impl std::str::FromStr for ProgramLine {
    type Err = ();
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let line = parse_numbers(&line);
        if line.len() == 4 {
            Ok(Self {
                opcode: line[0],
                parameters: (line[1], line[2], line[3]),
            })
        } else {
            Err(())
        }
    }
}

fn parse_numbers<T: std::str::FromStr>(input: &str) -> Vec<T> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d+").unwrap();
    }
    RE.captures_iter(input)
        .filter_map(|cap| Some(cap.get(0)?.as_str().parse().ok()?))
        .collect()
}

fn main() {
    let mut samples = Vec::new();
    let mut test_program = Vec::new();

    // Parse input
    {
        let mut before = None;
        let mut program_line = None;
        for line in stdin().lock().lines().filter_map(|l| l.ok()) {
            // Start parsing a sample
            if line.starts_with("Before:") {
                before = Some(parse_numbers(&line));
            } else if line.starts_with("After:") {
                // Everything is collected to build the sample, push it
                samples.push(Sample {
                    before: before.take().unwrap(),
                    after: parse_numbers(&line),
                    line: program_line.take().unwrap(),
                });
            }
            // Try to parse the program line
            else if let Ok(pl) = line.parse::<ProgramLine>() {
                // If we are building a Sample, store the program line for future usage
                if before.is_some() {
                    program_line = Some(pl);
                }
                // Else app the program line to the test program
                else {
                    test_program.push(pl);
                }
            }
        }
    }

    // Part 1
    {
        let matching_samples = samples
            .iter()
            .filter(|sample| {
                let matching_instructions = INSTRUCTIONS
                    .iter()
                    .filter(|instruction| instruction.test_sample(&sample))
                    .count();
                matching_instructions >= 3
            })
            .count();

        println!("Part 1: {}", matching_samples);
    }

    // Part 2
    {
        let mut instruction_indexes: Vec<Instructions> = vec![Instructions::Addr; 16];
        let mut unknown_instructions: Vec<_> = INSTRUCTIONS.to_vec();

        // While there is unknown_instructions, try to determine instruction opcodes
        while !unknown_instructions.is_empty() {
            for sample in &samples {
                let matching_instructions: Vec<_> = unknown_instructions
                    .iter()
                    .filter(|&instruction| instruction.test_sample(sample))
                    .cloned()
                    .collect();

                if matching_instructions.len() == 1 {
                    instruction_indexes[sample.line.opcode] = matching_instructions[0];
                    unknown_instructions.retain(|&i| i != matching_instructions[0]);
                }
            }
        }

        // Execute the program
        let mut registers = vec![0; 4];
        for program_line in &test_program {
            instruction_indexes[program_line.opcode].apply(program_line.parameters, &mut registers);
        }
        println!("Part 2: {}", registers[0]);
    }
}
