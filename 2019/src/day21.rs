#[cfg(test)]
mod tests {
    use crate::intcode::{Program, ProgramResult, Value};
    use crate::util;

    fn get_program() -> Program {
        util::input(21).next().expect("No input").parse().unwrap()
    }

    fn get_damages(result: &ProgramResult) -> Option<Value> {
        match result.get_output().last().cloned() {
            Some(last) if last > 255 => Some(last),
            _ => {
                result.print_ascii();
                None
            }
        }
    }

    #[test]
    fn part_1() {
        let mut program = get_program();

        // !A | !B | !C -> J
        program.run_str("NOT A J\n");
        program.run_str("NOT B T\n");
        program.run_str("OR T J\n");
        program.run_str("NOT C T\n");
        program.run_str("OR T J\n");

        // D & J -> J
        program.run_str("AND D J\n");

        let result = program.run_str("WALK\n");
        assert_eq!(get_damages(&result), util::answer(21, 1));
    }

    #[test]
    fn part_2() {
        let mut program = get_program();

        // Goal:
        // !(A & B & C) & D & (H | (E & (F | I))) -> J

        // F | I -> J
        program.run_str("OR F J\n");
        program.run_str("OR I J\n");

        // E & J -> J
        program.run_str("AND E J\n");

        // H | J -> J
        program.run_str("OR H J\n");

        // D & J -> J
        program.run_str("AND D J\n");

        // !(A & B & C) -> T
        program.run_str("OR A T\n");
        program.run_str("AND B T\n");
        program.run_str("AND C T\n");
        program.run_str("NOT T T\n");

        // T & J -> J
        program.run_str("AND T J\n");

        let result = program.run_str("RUN\n");

        assert_eq!(get_damages(&result), util::answer(21, 2));
    }
}
