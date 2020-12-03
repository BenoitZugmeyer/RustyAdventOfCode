fn run_slope<S: AsRef<str>, T: Iterator<Item = S>>(lines: T, dx: usize, dy: usize) -> usize {
    lines
        .step_by(dy)
        .scan(0, |position, line| {
            let result = line.as_ref().chars().nth(*position % line.as_ref().len());
            *position += dx;
            result
        })
        .filter(|ch| *ch == '#')
        .count()
}

#[allow(dead_code)]
fn part_1<T: Iterator<Item = String>>(lines: T) -> usize {
    run_slope(lines, 3, 1)
}

#[allow(dead_code)]
fn part_2<T: Iterator<Item = String>>(lines: T) -> usize {
    let lines: Vec<_> = lines.collect();
    run_slope(lines.iter(), 1, 1)
        * run_slope(lines.iter(), 3, 1)
        * run_slope(lines.iter(), 5, 1)
        * run_slope(lines.iter(), 7, 1)
        * run_slope(lines.iter(), 1, 2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test() {
        assert_eq!(part_1(util::example(3, 1)), 7);
    }

    #[test]
    fn test2() {
        assert_eq!(part_2(util::example(3, 1)), 336);
    }

    #[test]
    fn part_1_test() {
        assert_eq!(Some(part_1(util::input(3))), util::answer(3, 1));
    }

    #[test]
    fn part_2_test() {
        assert_eq!(Some(part_2(util::input(3))), util::answer(3, 2));
    }
}
