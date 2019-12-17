use std::convert::TryFrom;

pub type Value = i64;

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

    #[allow(dead_code)]
    pub fn get_output(self) -> Vec<Value> {
        match self {
            Self::Halt(result) | Self::NeedInput(result) => result,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    ip: usize,
    relative_base: Value,
    memory: Vec<Value>,
}

impl Program {
    pub fn new(memory: Vec<Value>) -> Self {
        Self {
            memory,
            ip: 0,
            relative_base: 0,
        }
    }

    #[allow(dead_code)]
    pub fn run(&mut self, input: &[Value]) -> ProgramResult {
        let mut input = input.iter();
        let mut output = Vec::new();
        loop {
            match self.read_memory(self.ip) % 100 {
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
                9 => {
                    self.relative_base += self.read(0);
                    self.ip += 2;
                }
                99 => {
                    return ProgramResult::Halt(output);
                }
                opcode => panic!("Invalid opcode {}", opcode),
            }
        }
    }

    fn write(&mut self, param: usize, value: Value) {
        self.write_memory(self.get_address(param), value);
    }

    fn read(&self, param: usize) -> Value {
        self.read_memory(self.get_address(param))
    }

    pub fn write_memory(&mut self, address: usize, value: Value) {
        if address >= self.memory.len() {
            self.memory.resize(address + 1, 0);
        }
        self.memory[address] = value;
    }

    fn read_memory(&self, address: usize) -> Value {
        self.memory.get(address).cloned().unwrap_or(0)
    }

    fn get_address(&self, param: usize) -> usize {
        let instruction = self.read_memory(self.ip);
        let mode = (instruction / (10_i64.pow(2 + u32::try_from(param).unwrap()))) % 10;
        let param_address = self.ip + 1 + param;
        match mode {
            0 => usize::try_from(self.read_memory(param_address)).unwrap(),
            1 => param_address,
            2 => usize::try_from(self.relative_base + self.read_memory(param_address)).unwrap(),
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
