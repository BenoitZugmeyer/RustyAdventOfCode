use std::convert::TryFrom;

type Value = i32;
type Program = Vec<Value>;

#[allow(dead_code)]
fn run_program(program: &mut Program, input: &[Value]) -> Vec<Value> {
    let mut input = input.iter();
    let mut output = Vec::new();
    let mut instruction_pointer = 0;
    loop {
        let instruction = program[instruction_pointer];
        let read = |param: usize| {
            let mode = (instruction / (10_i32.pow(2 + u32::try_from(param).unwrap()))) % 10;
            let v = program[instruction_pointer + 1 + param];
            match mode {
                0 => program[usize::try_from(v).unwrap()],
                1 => v,
                _ => panic!("Invalid mode {}", mode),
            }
        };
        let write = |program: &mut [Value], param: usize, value: Value| {
            program[usize::try_from(program[instruction_pointer + param + 1]).unwrap()] = value
        };
        let opcode = instruction % 100;

        match opcode {
            99 => break,
            1 => {
                let result = read(0) + read(1);
                write(program, 2, result);
                instruction_pointer += 4;
            }
            2 => {
                let result = read(0) * read(1);
                write(program, 2, result);
                instruction_pointer += 4;
            }
            3 => {
                write(program, 0, *input.next().expect("Failed to get some input"));
                instruction_pointer += 2;
            }
            4 => {
                output.push(read(0));
                instruction_pointer += 2;
            }
            5 => {
                if read(0) == 0 {
                    instruction_pointer += 3;
                } else {
                    instruction_pointer = usize::try_from(read(1)).unwrap();
                }
            }
            6 => {
                if read(0) == 0 {
                    instruction_pointer = usize::try_from(read(1)).unwrap();
                } else {
                    instruction_pointer += 3;
                }
            }
            7 => {
                let result = if read(0) < read(1) { 1 } else { 0 };
                write(program, 2, result);
                instruction_pointer += 4;
            }
            8 => {
                let result = if read(0) == read(1) { 1 } else { 0 };
                write(program, 2, result);
                instruction_pointer += 4;
            }
            _ => panic!("Invalid opcode {}", opcode),
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn run_program_test() {
        let mut program = vec![1002, 4, 3, 4, 33];
        run_program(&mut program, &[]);
        assert_eq!(program, vec![1002, 4, 3, 4, 99]);
    }

    fn get_program() -> Program {
        util::input(5)
            .next()
            .expect("No input")
            .split(',')
            .filter_map(|n| n.parse().ok())
            .collect()
    }

    #[test]
    fn part_1() {
        let mut program = get_program();
        assert_eq!(
            run_program(&mut program, &[1]).into_iter().last(),
            util::answer(5, 1)
        );
    }

    #[test]
    fn part_2() {
        let mut program = get_program();
        assert_eq!(
            run_program(&mut program, &[5]).into_iter().last(),
            util::answer(5, 2)
        );
    }
}
