use std::collections::HashSet;

#[allow(dead_code)]
fn part_1<T: Iterator<Item = String>>(lines: T) -> usize {
    lines
        .chain(std::iter::once(String::new()))
        .scan(HashSet::new(), |group, line| {
            Some(if line.is_empty() {
                Some(std::mem::replace(group, HashSet::new()))
            } else {
                for ch in line.chars() {
                    group.insert(ch);
                }
                None
            })
        })
        .flatten()
        .map(|group| group.len())
        .sum()
}

#[allow(dead_code)]
fn part_2<T: Iterator<Item = String>>(lines: T) -> usize {
    lines
        .chain(std::iter::once(String::new()))
        .scan(Option::<HashSet<char>>::None, |group, line| {
            Some(if line.is_empty() {
                group.take()
            } else {
                let line_group: HashSet<_> = line.chars().collect();
                if let Some(group) = group {
                    group.retain(|ch| line_group.contains(ch));
                } else {
                    *group = Some(line_group);
                }
                None
            })
        })
        .flatten()
        .map(|group| group.len())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test() {
        assert_eq!(part_1(util::example(6, 1)), 6);
    }

    #[test]
    fn test2() {
        assert_eq!(part_1(util::example(6, 2)), 11);
    }

    #[test]
    fn test3() {
        assert_eq!(part_2(util::example(6, 3)), 6);
    }

    #[test]
    fn part_1_test() {
        assert_eq!(Some(part_1(util::input(6))), util::answer(6, 1));
    }

    #[test]
    fn part_2_test() {
        assert_eq!(Some(part_2(util::input(6))), util::answer(6, 2));
    }
}
