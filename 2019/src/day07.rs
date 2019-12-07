use crate::intcode::{Program, ProgramResult};
use itertools::Itertools;

#[allow(dead_code)]
fn run_amplifiers(program: &Program) -> i32 {
    (0..5)
        .permutations(5)
        .map(|phase_settings| {
            let mut input_signal = 0;
            for phase_setting in phase_settings {
                let output = program.clone().run(&[phase_setting, input_signal]).unwrap();
                input_signal = output[0];
            }
            input_signal
        })
        .max()
        .unwrap()
}

#[allow(dead_code)]
fn run_amplifiers_with_feedback_loop(program: &Program) -> i32 {
    (5..10)
        .permutations(5)
        .map(|phase_settings| {
            let mut programs: Vec<_> = phase_settings
                .iter()
                .map(|phase_setting| {
                    let mut program = program.clone();
                    program.run(&[*phase_setting]);
                    program
                })
                .collect();
            let mut input_signal = 0;
            loop {
                for (i, program) in programs.iter_mut().enumerate() {
                    input_signal = match program.run(&[input_signal]) {
                        ProgramResult::Halt(output) => {
                            if i == 4 {
                                return output[0];
                            } else {
                                output[0]
                            }
                        }
                        ProgramResult::NeedInput(output) => output[0],
                    };
                }
            }
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn get_program() -> Program {
        util::input(7).next().expect("No input").parse().unwrap()
    }

    #[test]
    fn run_amplifiers_test() {
        assert_eq!(
            run_amplifiers(
                &"3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"
                    .parse()
                    .unwrap()
            ),
            43210
        );
        assert_eq!(
            run_amplifiers(
                &"3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"
                    .parse()
                    .unwrap()
            ),
            54321
        );
        assert_eq!(
            run_amplifiers(
                &"3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,\
                  31,31,4,31,99,0,0,0"
                    .parse()
                    .unwrap()
            ),
            65210
        );
    }

    #[test]
    fn part_1() {
        assert_eq!(Some(run_amplifiers(&get_program())), util::answer(7, 1));
    }

    #[test]
    fn run_amplifiers_with_feedback_loop_test() {
        assert_eq!(
            run_amplifiers_with_feedback_loop(
                &"3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,\
                  6,99,0,0,5"
                    .parse()
                    .unwrap()
            ),
            139_629_729
        );

        assert_eq!(
            run_amplifiers_with_feedback_loop(
                &"3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,\
                  1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,\
                  1005,56,6,99,0,0,0,0,10"
                    .parse()
                    .unwrap()
            ),
            18216
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(
            Some(run_amplifiers_with_feedback_loop(&get_program())),
            util::answer(7, 2)
        );
    }
}
