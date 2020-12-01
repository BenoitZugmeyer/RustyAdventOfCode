use itertools::Itertools;

fn part_1<T: Iterator<Item = i32>>(expenses: T) -> i32 {
    expenses
        .combinations(2)
        .find(|t| t[0] + t[1] == 2020)
        .map(|t| t[0] * t[1])
        .unwrap()
}

fn part_2<T: Iterator<Item = i32>>(expenses: T) -> i32 {
    let expenses: Vec<_> = expenses.collect();
    expenses
        .iter()
        .combinations(2)
        .find_map(|t| {
            if t[0] + t[1] < 2020 {
                expenses
                    .iter()
                    .find(|e| t[0] + t[1] + *e == 2020)
                    .map(|e| *t[0] * *t[1] * *e)
            } else {
                None
            }
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test() {
        assert_eq!(
            part_1([1721, 979, 366, 299, 675, 1456].iter().cloned()),
            514579
        );
    }

    #[test]
    fn part_1_test() {
        assert_eq!(
            Some(part_1(util::input(1).flat_map(|line| line.parse()))),
            util::answer(1, 1)
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            Some(part_2(util::input(1).flat_map(|line| line.parse()))),
            util::answer(1, 2)
        );
    }
}
