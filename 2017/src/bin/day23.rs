use std::io::{stdin, BufRead};
use std::collections::HashMap;

type Integer = i64;
type Register = char;
type Registers = HashMap<Register, Integer>;

fn first_char(v: &str) -> char {
    v.chars().nth(0).unwrap()
}

#[derive(Debug)]
enum Value {
    Register(Register),
    Integer(Integer),
}

impl Value {
    fn parse(v: &str) -> Value {
        v.parse::<Integer>()
            .map(|i| Value::Integer(i))
            .unwrap_or_else(|_| Value::Register(first_char(v)))
    }

    fn as_integer(&self, registers: &Registers) -> Integer {
        match *self {
            Value::Register(ref register) => registers.get(register).cloned().unwrap_or(0),
            Value::Integer(integer) => integer,
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Set(Register, Value),
    Sub(Register, Value),
    Mul(Register, Value),
    Jnz(Value, Value),
}

impl Instruction {
    fn parse(v: &str) -> Instruction {
        let chunks: Vec<&str> = v.trim().split(' ').collect();

        match chunks[0] {
            "set" => Instruction::Set(first_char(chunks[1]), Value::parse(chunks[2])),
            "sub" => Instruction::Sub(first_char(chunks[1]), Value::parse(chunks[2])),
            "mul" => Instruction::Mul(first_char(chunks[1]), Value::parse(chunks[2])),
            "jnz" => Instruction::Jnz(Value::parse(chunks[1]), Value::parse(chunks[2])),
            _ => unreachable!(),
        }
    }
}

trait Evaluator {
    fn get_next_instruction(&mut self) -> &mut Integer;
    fn get_registers(&mut self) -> &mut Registers;

    fn evaluate(&mut self, instructions: &[Instruction]) {
        loop {
            let instruction = {
                let next_instruction = *self.get_next_instruction();

                if next_instruction < 0 || next_instruction as usize >= instructions.len() {
                    return;
                }

                &instructions[next_instruction as usize]
            };

            match *instruction {
                Instruction::Set(register, ref value) => self.eval_set(register, value),
                Instruction::Sub(register, ref value) => self.eval_sub(register, value),
                Instruction::Mul(register, ref value) => self.eval_mul(register, value),
                Instruction::Jnz(ref condition, ref value) => self.eval_jnz(condition, value),
            };
        }
    }

    fn eval_set(&mut self, register: Register, value: &Value) {
        let integer = value.as_integer(self.get_registers());
        self.get_registers().insert(register, integer);
        *self.get_next_instruction() += 1;
    }

    fn eval_sub(&mut self, register: Register, value: &Value) {
        let integer = value.as_integer(self.get_registers());
        *self.get_registers().entry(register).or_insert(0) -= integer;
        *self.get_next_instruction() += 1;
    }

    fn _eval_mul(&mut self, register: Register, value: &Value) {
        let integer = value.as_integer(self.get_registers());
        *self.get_registers().entry(register).or_insert(0) *= integer;
        *self.get_next_instruction() += 1;
    }

    fn eval_mul(&mut self, register: Register, value: &Value) {
        self._eval_mul(register, value);
    }

    fn eval_jnz(&mut self, condition: &Value, value: &Value) {
        if condition.as_integer(self.get_registers()) != 0 {
            *self.get_next_instruction() += value.as_integer(self.get_registers());
        } else {
            *self.get_next_instruction() += 1;
        }
    }
}

struct Part1Evaluator {
    mul_instruction_invoked: u32,
    next_instruction: Integer,
    registers: Registers,
}

impl Evaluator for Part1Evaluator {
    fn get_next_instruction(&mut self) -> &mut Integer {
        &mut self.next_instruction
    }

    fn get_registers(&mut self) -> &mut Registers {
        &mut self.registers
    }

    fn eval_mul(&mut self, register: Register, value: &Value) {
        self.mul_instruction_invoked += 1;
        self._eval_mul(register, value);
    }
}

fn is_prime(n: u32) -> bool {
    if n % 2 == 0 {
        return false;
    }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    return true;
}

fn main() {
    let stdin = stdin();

    let instructions: Vec<Instruction> = stdin
        .lock()
        .lines()
        .filter_map(|l| l.ok())
        .map(|line| Instruction::parse(&line))
        .collect();

    {
        let mut program = Part1Evaluator {
            mul_instruction_invoked: 0,
            next_instruction: 0,
            registers: Registers::new(),
        };

        program.evaluate(&instructions);

        println!("Part 1: {}", program.mul_instruction_invoked);
    }

    {
        let h = (105700..=122700)
            .step_by(17)
            .filter(|&n| !is_prime(n))
            .count();
        println!("Part 2: {}", h);
    }
}
