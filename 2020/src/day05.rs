fn decode_id(s: &str) -> u32 {
    s.chars().fold(0, |total, ch| {
        (total << 1)
            + match ch {
                'B' | 'R' => 1,
                'F' | 'L' => 0,
                _ => panic!("Unexpected character {}", ch),
            }
    })
}

#[allow(dead_code)]
fn part_1<T: Iterator<Item = String>>(lines: T) -> Option<u32> {
    lines.map(|line| decode_id(&line)).max()
}

#[allow(dead_code)]
fn part_2<T: Iterator<Item = String>>(lines: T) -> Option<u32> {
    let mut seats: Vec<_> = lines.map(|line| decode_id(&line)).collect();
    seats.sort();
    seats
        .windows(2)
        .find(|couple| couple[0] + 1 != couple[1])
        .map(|couple| couple[0] + 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test() {
        assert_eq!(decode_id("BFFFBBFRRR"), 567);
        assert_eq!(decode_id("FFFBBBFRRR"), 119);
        assert_eq!(decode_id("BBFFBBFRLL"), 820);
    }

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(util::input(5)), util::answer(5, 1));
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(util::input(5)), util::answer(5, 2));
    }
}
