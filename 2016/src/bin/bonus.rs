// https://gist.github.com/topaz/15518587415ccd0468767aed4192bfd3
#[macro_use]
extern crate nom;

extern crate regex;
use regex::Regex;
use std::collections::BTreeMap;
use std::io::stdin;
use std::io::Read;

type Register = u8;

#[derive(Debug, Clone)]
enum Value {
    FromRegister(Register),
    Const(i32),
}

impl Value {
    fn get(&self, memory: &mut Memory) -> i32 {
        match *self {
            Value::FromRegister(r) => *memory.get(r),
            Value::Const(c) => c,
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Cpy(Value, Value),
    Inc(Value),
    Dec(Value),
    Jnz(Value, Value),
    Out(Value),
}

struct Memory {
    registers: BTreeMap<Register, i32>,
}

impl Memory {
    fn new() -> Memory {
        Memory {
            registers: BTreeMap::new(),
        }
    }

    fn get(&mut self, r: Register) -> &mut i32 {
        self.registers.entry(r).or_insert(0)
    }
}

named!(
    parse_number<i32>,
    do_parse!(
        minus: opt!(tag!("-"))
            >> n: take_while1!(nom::is_digit)
            >> (if minus == Some(b"-") { -1 } else { 1 }
                * n.iter()
                    .fold(0, |acc, item| acc * 10 + i32::from(item - b'0')))
    )
);

named!(
    parse_register<Register>,
    map!(take_while1!(nom::is_alphabetic), |bytes: &[u8]| bytes[0]
        - b'a')
);

named!(
    parse_value<Value>,
    alt!(
        map!(parse_number, |n| Value::Const(n)) | map!(parse_register, |n| Value::FromRegister(n))
    )
);

named!(
    parse_cpy<Instruction>,
    do_parse!(
        tag!("cpy ") >> n: parse_value >> tag!(" ") >> m: parse_value >> (Instruction::Cpy(n, m))
    )
);

named!(
    parse_dec<Instruction>,
    do_parse!(tag!("dec ") >> n: parse_value >> (Instruction::Dec(n)))
);

named!(
    parse_inc<Instruction>,
    do_parse!(tag!("inc ") >> n: parse_value >> (Instruction::Inc(n)))
);

named!(
    parse_out<Instruction>,
    do_parse!(tag!("out ") >> n: parse_value >> (Instruction::Out(n)))
);

named!(
    parse_jnz<Instruction>,
    do_parse!(
        tag!("jnz ") >> n: parse_value >> tag!(" ") >> m: parse_value >> (Instruction::Jnz(n, m))
    )
);

named!(
    parse_input(&[u8]) -> Vec<Instruction>,
    many0!(
        do_parse!(
            res: alt!( parse_cpy | parse_inc | parse_dec | parse_jnz | parse_out ) >>
            tag!("\n") >>
            (res)
        )
    )
);

fn exec(instructions: &[Instruction], memory: &mut Memory) -> String {
    let mut ptr = 0;
    let mut result = String::new();

    while ptr < instructions.len() {
        match instructions[ptr].clone() {
            Instruction::Cpy(ref v, Value::FromRegister(r)) => {
                let vv = v.get(memory);
                *memory.get(r) = vv;
                ptr += 1;
            }
            Instruction::Inc(Value::FromRegister(r)) => {
                *memory.get(r) += 1;
                ptr += 1;
            }
            Instruction::Dec(Value::FromRegister(r)) => {
                *memory.get(r) -= 1;
                ptr += 1;
            }
            Instruction::Jnz(ref a, ref b) => {
                let va = a.get(memory);
                let vb = b.get(memory);
                ptr = (ptr as i32 + if va == 0 { 1 } else { vb }) as usize;
            }
            Instruction::Out(ref v) => {
                let vv = v.get(memory);
                result.push(std::char::from_u32(vv as u32).unwrap());
                ptr += 1;
            }
            _ => {
                ptr += 1;
            }
        }
    }

    result
}

const ROWS: usize = 6;
const COLUMNS: usize = 50;
// const ROWS: usize = 3;
// const COLUMNS: usize = 7;

struct Screen {
    grid: [[bool; COLUMNS]; ROWS],
}

impl Screen {
    fn new() -> Self {
        Screen {
            grid: [[false; COLUMNS]; ROWS],
        }
    }
    fn rect(&mut self, a: usize, b: usize) {
        for x in 0..std::cmp::min(a, COLUMNS) {
            for y in 0..std::cmp::min(b, ROWS) {
                self.grid[y][x] = true;
            }
        }
    }
    fn rotate_row(&mut self, a: usize, b: usize) {
        let row = std::cmp::min(a, ROWS);
        let count = b % COLUMNS;
        let mut copy = [false; COLUMNS];
        copy.clone_from_slice(&self.grid[row]);

        for column in 0..COLUMNS {
            self.grid[row][column] = copy[(COLUMNS + column - count) % COLUMNS];
        }
    }
    fn rotate_column(&mut self, a: usize, b: usize) {
        let column = std::cmp::min(a, COLUMNS);
        let count = b % ROWS;
        let copy: Vec<_> = (0..ROWS).map(|row| self.grid[row][column]).collect();
        for row in 0..ROWS {
            self.grid[row][column] = copy[(ROWS + row - count) % ROWS];
        }
    }
}

impl std::fmt::Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for line in &self.grid {
            for v in line.iter() {
                if *v {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn print_to_screen(input: &str) -> Screen {
    let re = Regex::new(
        r"^(?x)(?:
        rect\s(\d+)x(\d+)
        |
        rotate\scolumn\sx=(\d+)\sby\s(\d+)
        |
        rotate\srow\sy=(\d+)\sby\s(\d+)
    )$",
    )
    .unwrap();
    let mut screen = Screen::new();
    for line in input.lines() {
        if let Some(cap) = re.captures(&line) {
            if let (Some(a), Some(b)) = (cap.at(1), cap.at(2)) {
                screen.rect(a.parse().unwrap(), b.parse().unwrap());
            } else if let (Some(a), Some(b)) = (cap.at(3), cap.at(4)) {
                screen.rotate_column(a.parse().unwrap(), b.parse().unwrap());
            } else if let (Some(a), Some(b)) = (cap.at(5), cap.at(6)) {
                screen.rotate_row(a.parse().unwrap(), b.parse().unwrap());
            }
        }
    }

    screen
}

fn main() {
    let mut input = Vec::new();
    stdin()
        .read_to_end(&mut input)
        .expect("Failed to read stdin");
    let (rest, instructions) = parse_input(&input).unwrap();
    if !rest.is_empty() {
        panic!(
            "Can't parse the rest of the input: {:?}",
            String::from_utf8_lossy(rest)
        );
    }

    let mut memory = Memory::new();

    let output = exec(&instructions, &mut memory);
    print!("{}", print_to_screen(&output));
}
