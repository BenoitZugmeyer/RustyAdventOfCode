use std::io::{stdin, BufRead};
use std::collections::{HashMap, VecDeque};

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
    Snd(Value),
    Set(Register, Value),
    Add(Register, Value),
    Mul(Register, Value),
    Mod(Register, Value),
    Rcv(Register),
    Jgz(Value, Value),
}

impl Instruction {
    fn parse(v: &str) -> Instruction {
        let chunks: Vec<&str> = v.trim().split(' ').collect();

        match chunks[0] {
            "snd" => Instruction::Snd(Value::parse(chunks[1])),
            "set" => Instruction::Set(first_char(chunks[1]), Value::parse(chunks[2])),
            "add" => Instruction::Add(first_char(chunks[1]), Value::parse(chunks[2])),
            "mul" => Instruction::Mul(first_char(chunks[1]), Value::parse(chunks[2])),
            "mod" => Instruction::Mod(first_char(chunks[1]), Value::parse(chunks[2])),
            "rcv" => Instruction::Rcv(first_char(chunks[1])),
            "jgz" => Instruction::Jgz(Value::parse(chunks[1]), Value::parse(chunks[2])),
            _ => unreachable!(),
        }
    }
}

trait Evaluator {
    fn eval_rcv(&mut self, register: Register);
    fn eval_snd(&mut self, value: &Value);
    fn finished(&self) -> bool;
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
                Instruction::Snd(ref value) => self.eval_snd(value),
                Instruction::Set(register, ref value) => self.eval_set(register, value),
                Instruction::Add(register, ref value) => self.eval_add(register, value),
                Instruction::Mul(register, ref value) => self.eval_mul(register, value),
                Instruction::Mod(register, ref value) => self.eval_mod(register, value),
                Instruction::Rcv(register) => self.eval_rcv(register),
                Instruction::Jgz(ref condition, ref value) => self.eval_jgz(condition, value),
            };

            if self.finished() {
                return;
            }
        }
    }

    fn eval_set(&mut self, register: Register, value: &Value) {
        let integer = value.as_integer(self.get_registers());
        self.get_registers().insert(register, integer);
        *self.get_next_instruction() += 1;
    }

    fn eval_add(&mut self, register: Register, value: &Value) {
        let integer = value.as_integer(self.get_registers());
        *self.get_registers().entry(register).or_insert(0) += integer;
        *self.get_next_instruction() += 1;
    }

    fn eval_mul(&mut self, register: Register, value: &Value) {
        let integer = value.as_integer(self.get_registers());
        *self.get_registers().entry(register).or_insert(0) *= integer;
        *self.get_next_instruction() += 1;
    }

    fn eval_mod(&mut self, register: Register, value: &Value) {
        let integer = value.as_integer(self.get_registers());
        *self.get_registers().entry(register).or_insert(0) %= integer;
        *self.get_next_instruction() += 1;
    }

    fn eval_jgz(&mut self, condition: &Value, value: &Value) {
        if condition.as_integer(self.get_registers()) > 0 {
            *self.get_next_instruction() += value.as_integer(self.get_registers());
        } else {
            *self.get_next_instruction() += 1;
        }
    }
}

struct Part1Evaluator {
    last_sound_checked: Option<Integer>,
    got_rcv: bool,
    next_instruction: Integer,
    registers: Registers,
}

impl Evaluator for Part1Evaluator {
    fn eval_snd(&mut self, value: &Value) {
        self.last_sound_checked = Some(value.as_integer(self.get_registers()));
        *self.get_next_instruction() += 1;
    }

    fn eval_rcv(&mut self, _: Register) {
        self.got_rcv = true;
        *self.get_next_instruction() += 1;
    }

    fn finished(&self) -> bool {
        self.got_rcv
    }

    fn get_next_instruction(&mut self) -> &mut Integer {
        &mut self.next_instruction
    }

    fn get_registers(&mut self) -> &mut Registers {
        &mut self.registers
    }
}


struct Part2Evaluator {
    next_instruction: Integer,
    registers: Registers,
    waiting_for_value: bool,
    sent_counter: u32,
    input: Option<VecDeque<Integer>>,
    output: Option<VecDeque<Integer>>,
}

impl Part2Evaluator {
    fn new(pid: Integer) -> Self {
        let mut registers = Registers::new();
        registers.insert('p', pid);
        Part2Evaluator {
            next_instruction: 0,
            registers,
            waiting_for_value: false,
            sent_counter: 0,
            input: None,
            output: None,
        }
    }
}

impl Evaluator for Part2Evaluator {
    fn eval_snd(&mut self, value: &Value) {
        let integer = value.as_integer(self.get_registers());
        self.output.as_mut().unwrap().push_back(integer);
        self.sent_counter += 1;
        *self.get_next_instruction() += 1;
    }

    fn eval_rcv(&mut self, register: Register) {
        if let Some(integer) = self.input.as_mut().unwrap().pop_front() {
            self.get_registers().insert(register, integer);
            *self.get_next_instruction() += 1;
        } else {
            self.waiting_for_value = true;
        }
    }

    fn finished(&self) -> bool {
        self.waiting_for_value
    }

    fn get_next_instruction(&mut self) -> &mut Integer {
        &mut self.next_instruction
    }

    fn get_registers(&mut self) -> &mut Registers {
        &mut self.registers
    }
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
            last_sound_checked: None,
            got_rcv: false,
            next_instruction: 0,
            registers: Registers::new(),
        };

        program.evaluate(&instructions);

        println!("Part 1: {}", program.last_sound_checked.unwrap());
    }

    {
        let mut programs = [Part2Evaluator::new(0), Part2Evaluator::new(1)];

        programs[0].input = Some(VecDeque::new());
        programs[0].output = Some(VecDeque::new());

        let mut index = 0;
        loop {
            if programs[index].waiting_for_value &&
                programs[index].input.as_ref().unwrap().is_empty()
            {
                break;
            }

            programs[index].waiting_for_value = false;
            programs[index].evaluate(&instructions);
            if !programs[index].waiting_for_value {
                break;
            }

            programs[1 - index].input = programs[index].output.take();
            programs[1 - index].output = programs[index].input.take();

            index = (index + 1) % 2;
        }

        println!("Part 2: {}", programs[1].sent_counter);
    }
}
