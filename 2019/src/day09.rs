#[cfg(test)]
mod tests {
    use crate::intcode::Program;
    use crate::util;

    fn get_program() -> Program {
        util::input(9).next().expect("No input").parse().unwrap()
    }

    #[test]
    fn complete_lintcode_test() {
        assert_eq!(
            "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"
                .parse::<Program>()
                .unwrap()
                .run(&[])
                .unwrap(),
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        );
        assert_eq!(
            "1102,34915192,34915192,7,4,7,99,0"
                .parse::<Program>()
                .unwrap()
                .run(&[])
                .unwrap(),
            vec![1_219_070_632_396_864]
        );
        assert_eq!(
            "104,1125899906842624,99"
                .parse::<Program>()
                .unwrap()
                .run(&[])
                .unwrap(),
            vec![1_125_899_906_842_624]
        );
    }

    #[test]
    fn part_1() {
        let mut program = get_program();
        let result = program.run(&[1]).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(Some(result[0]), util::answer(9, 1));
    }

    #[test]
    fn part_2() {
        let mut program = get_program();
        let result = program.run(&[2]).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(Some(result[0]), util::answer(9, 2));
    }
}
