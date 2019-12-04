use itertools::Itertools;

struct Digits {
    n: u32,
}
impl Digits {
    fn new(n: u32) -> Self {
        Self { n }
    }
}
impl Iterator for Digits {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.n == 0 {
            None
        } else {
            let rem = self.n % 10;
            self.n /= 10;
            Some(rem)
        }
    }
}

#[allow(dead_code)]
fn are_digits_increasing(n: u32) -> bool {
    Digits::new(n).tuple_windows().all(|(a, b)| a >= b)
}

#[allow(dead_code)]
fn have_same_digits_pair(n: u32) -> bool {
    Digits::new(n).tuple_windows().any(|(a, b)| a == b)
}

#[allow(dead_code)]
fn have_same_digits_pair2(n: u32) -> bool {
    let mut pairs = Digits::new(n)
        .tuple_windows()
        .map(|(a, b)| a == b)
        .peekable();
    let mut had_pair = false;
    while let Some(is_pair) = pairs.next() {
        let will_have_pair = pairs.peek() == Some(&true);
        if !had_pair && is_pair && !will_have_pair {
            return true;
        }
        had_pair = is_pair;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn iter_digits_test() {
        assert_eq!(Digits::new(12345).collect::<Vec<_>>(), vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn are_digits_increasing_test() {
        assert_eq!(are_digits_increasing(1234), true);
        assert_eq!(are_digits_increasing(1111), true);
        assert_eq!(are_digits_increasing(4333), false);
    }

    #[test]
    fn have_same_digits_pair_test() {
        assert_eq!(have_same_digits_pair(1123), true);
        assert_eq!(have_same_digits_pair(1002), true);
        assert_eq!(have_same_digits_pair(1234), false);
    }

    #[test]
    fn have_same_digits_pair_test2() {
        assert_eq!(have_same_digits_pair2(1123), true);
        assert_eq!(have_same_digits_pair2(1002), true);
        assert_eq!(have_same_digits_pair2(1234), false);
        assert_eq!(have_same_digits_pair2(1114), false);
        assert_eq!(have_same_digits_pair2(112_233), true);
        assert_eq!(have_same_digits_pair2(123_444), false);
        assert_eq!(have_same_digits_pair2(111_122), true);
    }

    fn input_numbers() -> (u32, u32) {
        let numbers: Vec<u32> = util::input(4)
            .next()
            .expect("No input")
            .split('-')
            .filter_map(|n| n.parse().ok())
            .collect();
        (numbers[0], numbers[1])
    }

    #[test]
    fn part_1() {
        let (start, end) = input_numbers();
        assert_eq!(
            Some(
                (start..end)
                    .filter(|n| are_digits_increasing(*n))
                    .filter(|n| have_same_digits_pair(*n))
                    .count()
            ),
            util::answer(4, 1)
        );
    }

    #[test]
    fn part_2() {
        let (start, end) = input_numbers();
        assert_eq!(
            Some(
                (start..end)
                    .filter(|n| are_digits_increasing(*n))
                    .filter(|n| have_same_digits_pair2(*n))
                    .count()
            ),
            util::answer(4, 2)
        );
    }
}
