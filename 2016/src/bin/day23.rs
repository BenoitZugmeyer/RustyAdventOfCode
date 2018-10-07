#[macro_use]
extern crate nom;

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
    Tgl(Value),
}

impl Instruction {
    fn toggle(&mut self) {
        *self = match *self {
            Instruction::Inc(ref a) => Instruction::Dec(a.clone()),
            Instruction::Tgl(ref a) |
            Instruction::Dec(ref a) => Instruction::Inc(a.clone()),
            Instruction::Jnz(ref a, ref b) => Instruction::Cpy(a.clone(), b.clone()),
            Instruction::Cpy(ref a, ref b) => Instruction::Jnz(a.clone(), b.clone()),
        }
    }
}

struct Memory {
    registers: BTreeMap<Register, i32>,
}

impl Memory {
    fn new() -> Memory {
        Memory { registers: BTreeMap::new() }
    }

    fn get(&mut self, r: Register) -> &mut i32 {
        self.registers.entry(r).or_insert(0)
    }

    fn reset(&mut self) {
        self.registers.clear();
    }
}

named!(
    parse_number<i32>,
    do_parse!(
        minus: opt!(tag!("-")) >>
        n: take_while1!(nom::is_digit) >>
        (if minus == Some(b"-") { -1 } else { 1 } *
         n.iter().fold(0, |acc, item| acc * 10 + (item - b'0') as i32))
    )
);

named!(
    parse_register<Register>,
    map!(take_while1!(nom::is_alphabetic), |bytes: &[u8]| bytes[0] - b'a')
);

named!(
    parse_value<Value>,
    alt!(
        map!(parse_number, |n| Value::Const(n))
        |
        map!(parse_register, |n| Value::FromRegister(n))
    )
);

named!(
    parse_cpy<Instruction>,
    do_parse!(
        tag!("cpy ") >>
        n: parse_value >>
        tag!(" ") >>
        m: parse_value >>
        (Instruction::Cpy(n, m))
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
    parse_tgl<Instruction>,
    do_parse!(tag!("tgl ") >> n: parse_value >> (Instruction::Tgl(n)))
);

named!(
    parse_jnz<Instruction>,
    do_parse!(
        tag!("jnz ") >>
        n: parse_value >>
        tag!(" ") >>
        m: parse_value >>
        (Instruction::Jnz(n, m))
    )
);

named!(
    parse_input(&[u8]) -> Vec<Instruction>,
    many0!(
        do_parse!(
            res: alt!( parse_cpy | parse_inc | parse_dec | parse_jnz | parse_tgl ) >>
            tag!("\n") >>
            (res)
        )
    )
);

fn exec(instructions: &mut [Instruction], memory: &mut Memory) {
    let mut ptr = 0;

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
            Instruction::Tgl(ref v) => {
                let index = (v.get(memory) + ptr as i32) as usize;
                if index < instructions.len() {
                    instructions[index].toggle();
                }
                ptr += 1;
            }
            _ => {
                ptr += 1;
            }
        }
    }
}



fn main() {
    let mut input = Vec::new();
    stdin().read_to_end(&mut input).expect("Failed to read stdin");
    let (rest, instructions) = parse_input(&input).unwrap();
    if !rest.is_empty() {
        panic!("Can't parse the rest of the input: {:?}", String::from_utf8_lossy(rest));
    }

    let mut memory = Memory::new();
    *memory.get(0) = 7;

    exec(&mut instructions.clone(), &mut memory);
    println!("Part 1: {:?}", memory.get(0));

    memory.reset();
    *memory.get(0) = 12;

    exec(&mut instructions.clone(), &mut memory);
    println!("Part 2: {:?}", memory.get(0));
}
