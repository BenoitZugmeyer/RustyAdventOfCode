use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

#[allow(dead_code)]
fn part_1<T: Iterator<Item = String>>(lines: T) -> usize {
    let rules = parse_rules(lines);

    let first = "shiny gold".to_string();
    let mut bags = VecDeque::new();
    bags.push_front(&first);

    let mut all_parents = HashSet::new();
    while let Some(bag) = bags.pop_back() {
        for (parent, children) in rules.iter() {
            if all_parents.contains(parent) {
                continue;
            }
            if children.contains_key(bag) {
                bags.push_front(parent);
                all_parents.insert(parent);
            }
        }
    }

    all_parents.len()
}

#[allow(dead_code)]
fn part_2<T: Iterator<Item = String>>(lines: T) -> u32 {
    let rules = parse_rules(lines);

    let first = "shiny gold".to_string();
    let mut bags = VecDeque::new();
    bags.push_front((&first, 1));

    let mut all_child_count = 0;
    while let Some((bag, count)) = bags.pop_back() {
        let children = &rules[bag];

        for (child, child_count) in children.iter() {
            all_child_count += child_count * count;
            bags.push_front((child, child_count * count));
        }
    }

    all_child_count
}

fn parse_rules<T: Iterator<Item = String>>(lines: T) -> HashMap<String, HashMap<String, u32>> {
    lines
        .map(|line| {
            let mut words = line.split(|ch| ch == ' ' || ch == '.' || ch == ',');
            let words = words.by_ref();
            let color = words.take(2).join(" ");
            assert_eq!(words.next(), Some("bags"));
            assert_eq!(words.next(), Some("contain"));
            let mut children = HashMap::new();
            loop {
                let number = match words.next() {
                    Some("no") => break,
                    Some("bag") => continue,
                    Some("bags") => continue,
                    Some("") => continue,
                    Some(number) => number,
                    None => break,
                };
                let number: u32 = number.parse().unwrap();
                let child_color = words.take(2).join(" ");
                children.insert(child_color, number);
            }
            (color, children)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test() {
        assert_eq!(part_1(util::example(7, 1)), 4);
    }

    #[test]
    fn test2() {
        assert_eq!(part_2(util::example(7, 1)), 32);
    }

    #[test]
    fn test3() {
        assert_eq!(part_2(util::example(7, 2)), 126);
    }

    #[test]
    fn part_1_test() {
        assert_eq!(Some(part_1(util::input(7))), util::answer(7, 1));
    }

    #[test]
    fn part_2_test() {
        assert_eq!(Some(part_2(util::input(7))), util::answer(7, 2));
    }
}
