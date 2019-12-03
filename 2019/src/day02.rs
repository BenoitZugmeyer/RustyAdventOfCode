type Value = u32;
type Program = Vec<Value>;
type ProgramSlice = [Value];

fn run_program(program: &mut Program) {
    let mut instruction_pointer = 0;
    loop {
        match program[instruction_pointer] {
            99 => return,
            instruction => {
                let op1 = program[instruction_pointer + 1] as usize;
                let op2 = program[instruction_pointer + 2] as usize;
                let output = program[instruction_pointer + 3] as usize;
                match instruction {
                    1 => program[output] = program[op1] + program[op2],
                    2 => program[output] = program[op1] * program[op2],
                    _ => panic!("Invalid instruction {}", instruction),
                }
            }
        }
        instruction_pointer += 4;
    }
}

fn run_program_with_input(mut program: Program, noun: Value, verb: Value) -> Value {
    program[1] = noun;
    program[2] = verb;
    run_program(&mut program);
    program[0]
}

#[allow(dead_code)]
fn find_verb_noun(program: &ProgramSlice) -> Value {
    for verb in 0..=99 {
        for noun in 0..=99 {
            if run_program_with_input(program.to_owned(), noun, verb) == 19_690_720 {
                return 100 * noun + verb;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn run_program_test_1() {
        let mut p = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        run_program(&mut p);
        assert_eq!(p, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
    }

    #[test]
    fn run_program_test_2() {
        let mut p = vec![1, 0, 0, 0, 99];
        run_program(&mut p);
        assert_eq!(p, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn run_program_test_3() {
        let mut p = vec![2, 3, 0, 3, 99];
        run_program(&mut p);
        assert_eq!(p, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn run_program_test_4() {
        let mut p = vec![2, 4, 4, 5, 99, 0];
        run_program(&mut p);
        assert_eq!(p, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn run_program_test_5() {
        let mut p = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        run_program(&mut p);
        assert_eq!(p, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    fn get_program() -> Program {
        util::input(2)
            .next()
            .expect("No input")
            .split(',')
            .filter_map(|n| n.parse().ok())
            .collect()
    }
    #[test]
    fn part_1() {
        assert_eq!(
            Some(run_program_with_input(get_program(), 12, 2)),
            util::answer(2, 1)
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(Some(find_verb_noun(&get_program())), util::answer(2, 2));
    }
}
