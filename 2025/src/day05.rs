use std::ops::Range;

fn part_1(database: Database) -> usize {
    database
        .ids
        .iter()
        .filter(|id| database.ranges.iter().any(|range| range.contains(id)))
        .count()
}

fn part_2(mut database: Database) -> u64 {
    database.ranges.sort_by_key(|range| range.start);

    let mut min = 0;
    database
        .ranges
        .iter()
        .map(|range| {
            let adjusted_range = range.start.max(min)..range.end.max(min);
            min = adjusted_range.end;
            adjusted_range.end - adjusted_range.start
        })
        .sum()
}

#[derive(Debug, Default)]
struct Database {
    ranges: Vec<Range<u64>>,
    ids: Vec<u64>,
}

fn parse_database(lines: impl Iterator<Item = String>) -> Database {
    let mut database = Database::default();

    for line in lines {
        if let Some((start, end)) = line.split_once('-') {
            let start = start.parse().unwrap();
            let end: u64 = end.parse().unwrap();
            database.ranges.push(start..(end + 1));
        } else if let Ok(id) = line.parse() {
            database.ids.push(id);
        }
    }

    database
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;
    const DAY: u8 = 5;

    #[test]
    fn test1() {
        assert_eq!(part_1(parse_database(util::example(DAY, 1))), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(part_2(parse_database(util::example(DAY, 2))), 14);
    }

    #[test]
    fn part_1_test() {
        assert_eq!(
            Some(part_1(parse_database(util::input(DAY)))),
            util::answer(DAY, 1)
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            Some(part_2(parse_database(util::input(DAY)))),
            util::answer(DAY, 2)
        );
    }
}
