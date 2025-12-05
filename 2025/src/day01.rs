#[allow(dead_code)]
fn part_1<T: Iterator<Item = i32>>(moves: T) -> i32 {
    let mut position = 50i32;
    let mut count = 0;
    for m in moves {
        position += m;
        position = position.rem_euclid(100);
        if position == 0 {
            count += 1
        }
    }
    return count;
}

#[allow(dead_code)]
fn part_2<T: Iterator<Item = i32>>(moves: T) -> i32 {
    let mut position = 50i32;
    let mut count = 0;
    for m in moves {
        count += if position == 0 {
            m.abs() / 100
        } else if m < 0 {
            (100 - (position + m)) / 100
        } else {
            (position + m) / 100
        };
        position += m;
        position = position.rem_euclid(100);
    }
    return count;
}

#[allow(dead_code)]
fn parse_line(line: impl AsRef<str>) -> i32 {
    let line = line.as_ref();
    let n: i32 = line[1..].parse().unwrap_or_default();
    if line.starts_with('L') {
        -n
    } else {
        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test() {
        assert_eq!(part_1(util::example(1, 1).map(parse_line)), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(part_2(util::example(1, 1).map(parse_line)), 6);
    }

    #[test]
    fn part_1_test() {
        assert_eq!(
            Some(part_1(util::input(1).map(parse_line))),
            util::answer(1, 1)
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            Some(part_2(util::input(1).map(parse_line))),
            util::answer(1, 2)
        );
    }
}
