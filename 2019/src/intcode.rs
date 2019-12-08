use std::convert::TryFrom;

pub type Value = i32;

#[derive(Debug)]
pub enum ProgramResult {
    Halt(Vec<Value>),
    NeedInput(Vec<Value>),
}

impl ProgramResult {
    #[allow(dead_code)]
    pub fn unwrap(self) -> Vec<Value> {
        match self {
            Self::Halt(result) => result,
            Self::NeedInput(_) => panic!("The program is waiting for input"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    ip: usize,
    memory: Vec<Value>,
}

impl Program {
    pub fn new(memory: Vec<Value>) -> Self {
        Self { memory, ip: 0 }
    }

    #[allow(dead_code)]
    pub fn run(&mut self, input: &[Value]) -> ProgramResult {
        let mut input = input.iter();
        let mut output = Vec::new();
        loop {
            match self.memory[self.ip] % 100 {
                1 => {
                    let result = self.read(0) + self.read(1);
                    self.write(2, result);
                    self.ip += 4;
                }
                2 => {
                    let result = self.read(0) * self.read(1);
                    self.write(2, result);
                    self.ip += 4;
                }
                3 => {
                    if let Some(input) = input.next() {
                        self.write(0, *input);
                        self.ip += 2;
                    } else {
                        return ProgramResult::NeedInput(output);
                    }
                }
                4 => {
                    output.push(self.read(0));
                    self.ip += 2;
                }
                5 => {
                    if self.read(0) == 0 {
                        self.ip += 3;
                    } else {
                        self.ip = usize::try_from(self.read(1)).unwrap();
                    }
                }
                6 => {
                    if self.read(0) == 0 {
                        self.ip = usize::try_from(self.read(1)).unwrap();
                    } else {
                        self.ip += 3;
                    }
                }
                7 => {
                    let result = if self.read(0) < self.read(1) { 1 } else { 0 };
                    self.write(2, result);
                    self.ip += 4;
                }
                8 => {
                    let result = if self.read(0) == self.read(1) { 1 } else { 0 };
                    self.write(2, result);
                    self.ip += 4;
                }
                99 => {
                    return ProgramResult::Halt(output);
                }
                opcode => panic!("Invalid opcode {}", opcode),
            }
        }
    }

    fn write(&mut self, param: usize, value: Value) {
        let address = usize::try_from(self.memory[self.ip + param + 1]).unwrap();
        self.memory[address] = value;
    }

    fn read(&self, param: usize) -> Value {
        let instruction = self.memory[self.ip];
        let mode = (instruction / (10_i32.pow(2 + u32::try_from(param).unwrap()))) % 10;
        let v = self.memory[self.ip + 1 + param];
        match mode {
            0 => self.memory[usize::try_from(v).unwrap()],
            1 => v,
            _ => panic!("Invalid mode {}", mode),
        }
    }
}

impl std::str::FromStr for Program {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut memory = Vec::new();
        for value in s.split(',') {
            memory.push(value.parse()?);
        }
        Ok(Self::new(memory))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_program_test() {
        let mut program = Program::new(vec![1002, 4, 3, 4, 33]);
        program.run(&[]);
        assert_eq!(program.memory, vec![1002, 4, 3, 4, 99]);
    }
}
