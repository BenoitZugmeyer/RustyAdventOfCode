
#[cfg(test)]
mod tests {
    use crate::util;
    use crate::intcode::Program;

    fn get_program() -> Program {
        util::input(5).next().expect("No input").parse().unwrap()
    }

    #[test]
    fn part_1() {
        let mut program = get_program();
        assert_eq!(
            program.run(&[1]).unwrap().into_iter().last(),
            util::answer(5, 1)
        );
    }

    #[test]
    fn part_2() {
        let mut program = get_program();
        assert_eq!(
            program.run(&[5]).unwrap().into_iter().last(),
            util::answer(5, 2)
        );
    }
}
