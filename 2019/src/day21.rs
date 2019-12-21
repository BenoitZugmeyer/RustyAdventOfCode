#[cfg(test)]
mod tests {
    use crate::intcode::{Program, Value};
    use crate::util;

    fn get_program() -> Program {
        util::input(21).next().expect("No input").parse().unwrap()
    }

    fn encode_str(s: &str) -> Vec<Value> {
        s.bytes().map(i64::from).collect()
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn print_output(output: &[Value]) {
        for v in output {
            print!("{}", (*v as u8) as char);
        }
    }

    fn get_damages(output: &[Value]) -> Option<Value> {
        match output.last().cloned() {
            Some(last) if last > 255 => Some(last),
            _ => {
                print_output(&output);
                None
            }
        }
    }

    #[test]
    fn part_1() {
        let mut program = get_program();

        // !A | !B | !C -> J
        program.run(&encode_str("NOT A J\n"));
        program.run(&encode_str("NOT B T\n"));
        program.run(&encode_str("OR T J\n"));
        program.run(&encode_str("NOT C T\n"));
        program.run(&encode_str("OR T J\n"));

        // D & J -> J
        program.run(&encode_str("AND D J\n"));

        let output = program.run(&encode_str("WALK\n")).get_output();
        assert_eq!(get_damages(&output), util::answer(21, 1));
    }

    #[test]
    fn part_2() {
        let mut program = get_program();

        // Goal:
        // !(A & B & C) & D & (H | (E & (F | I))) -> J

        // F | I -> J
        program.run(&encode_str("OR F J\n"));
        program.run(&encode_str("OR I J\n"));

        // E & J -> J
        program.run(&encode_str("AND E J\n"));

        // H | J -> J
        program.run(&encode_str("OR H J\n"));

        // D & J -> J
        program.run(&encode_str("AND D J\n"));

        // !(A & B & C) -> T
        program.run(&encode_str("OR A T\n"));
        program.run(&encode_str("AND B T\n"));
        program.run(&encode_str("AND C T\n"));
        program.run(&encode_str("NOT T T\n"));

        // T & J -> J
        program.run(&encode_str("AND T J\n"));

        let output = program.run(&encode_str("RUN\n")).get_output();

        assert_eq!(get_damages(&output), util::answer(21, 2));
    }
}
