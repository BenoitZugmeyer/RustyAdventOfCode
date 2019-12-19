#[cfg(test)]
mod tests {
    use crate::intcode::{Program, Value};
    use crate::util;
    use itertools::iproduct;

    fn get_program() -> Program {
        util::input(19).next().expect("No input").parse().unwrap()
    }

    fn is_in_beam(program: &Program, x: Value, y: Value) -> bool {
        program.clone().run(&[x, y]).get_output()[0] == 1
    }

    #[test]
    fn part_1() {
        let program = get_program();
        assert_eq!(
            Some(
                iproduct!(0..50, 0..50)
                    .filter(|(x, y)| is_in_beam(&program, *x, *y))
                    .count()
            ),
            util::answer(19, 1)
        );
    }

    #[test]
    #[allow(clippy::maybe_infinite_iter)]
    fn part_2() {
        let program = get_program();
        let ship_size = 100;
        let mut x = 0;
        let mut y = 0;
        while !is_in_beam(&program, x + ship_size - 1, y - ship_size + 1) {
            y += 1;
            x = (x..).find(|x| is_in_beam(&program, *x, y)).unwrap();
        }
        y -= ship_size - 1;
        assert_eq!(Some(x * 10_000 + y), util::answer(19, 2));
    }
}
