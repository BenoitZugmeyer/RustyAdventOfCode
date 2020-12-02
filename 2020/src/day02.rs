fn parse<T: Iterator<Item = String>, F: Fn(usize, usize, char, &str) -> bool>(
    lines: T,
    validator: F,
) -> usize {
    lines
        .filter(|line| {
            dbg!(&line);
            let mut iter = line.split(|ch| ch == ' ' || ch == ':' || ch == '-');
            let min: usize = iter.next().unwrap().parse().unwrap();
            let max: usize = iter.next().unwrap().parse().unwrap();
            let ch: char = iter.next().unwrap().chars().next().unwrap();
            iter.next();
            let password = iter.next().unwrap();
            validator(min, max, ch, password)
        })
        .count()
}

#[allow(dead_code)]
fn part_1<T: Iterator<Item = String>>(lines: T) -> usize {
    parse(lines, |min, max, ch, password| {
        let count = password.chars().filter(|chp| *chp == ch).count();
        min <= count && count <= max
    })
}

#[allow(dead_code)]
fn part_2<T: Iterator<Item = String>>(lines: T) -> usize {
    parse(lines, |position1, position2, ch, password| {
        matches!(
            (
                password.chars().nth(position1 - 1),
                password.chars().nth(position2 - 1),
            ),
            (Some(chp), Some(other)) | (Some(other), Some(chp)) if other != ch && ch == chp
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test() {
        assert_eq!(part_1(util::example(2, 1)), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(part_2(util::example(2, 1)), 1);
    }

    #[test]
    fn part_1_test() {
        assert_eq!(Some(part_1(util::input(2))), util::answer(2, 1));
    }

    #[test]
    fn part_2_test() {
        assert_eq!(Some(part_2(util::input(2))), util::answer(2, 2));
    }
}
