use std::collections::HashSet;

type Range = (u64, u64);

#[allow(dead_code)]
fn part_1(ranges: &[Range]) -> u64 {
    let repeatitions = 2;

    let mut sum: u64 = 0;
    for range in ranges {
        sum += count_repeated_in_range(*range, repeatitions, &mut HashSet::new());
    }

    sum
}

#[allow(dead_code)]
fn part_2(ranges: &[Range]) -> u64 {
    let mut sum: u64 = 0;
    for range in &ranges[0..] {
        let mut previous: HashSet<u64> = HashSet::new();
        for repeatitions in 2..=10 {
            sum += count_repeated_in_range(*range, repeatitions, &mut previous);
        }
    }

    sum
}

fn count_repeated_in_range(
    (start, end): Range,
    repetitions: u32,
    previous: &mut HashSet<u64>,
) -> u64 {
    let mut sum: u64 = 0;
    let part_digits = (((start.ilog10() + 1) as f32) / repetitions as f32).ceil() as u32;
    let start_first_part = start / 10u64.pow(part_digits * repetitions);

    let mut repeated_part = start_first_part.max(1);
    loop {
        let repeated_part_digits = repeated_part.ilog10() + 1;
        let repeated = (0..repetitions).fold(0, |acc, pow| {
            repeated_part * 10u64.pow(repeated_part_digits * pow) + acc
        });

        if repeated > end {
            break;
        }

        repeated_part += 1;

        if repeated >= start && !previous.contains(&repeated) {
            sum += repeated;
            previous.insert(repeated);
        }
    }
    sum
}

#[allow(dead_code)]
fn parse_ranges(lines: impl Iterator<Item = String>) -> Vec<Range> {
    let content = lines.collect::<String>();

    content
        .split(',')
        .map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;
    const DAY: u8 = 2;

    #[test]
    fn test1() {
        assert_eq!(part_1(&parse_ranges(util::example(DAY, 1))), 1227775554);
    }

    #[test]
    fn test2() {
        assert_eq!(part_2(&parse_ranges(util::example(DAY, 1))), 4174379265);
    }

    #[test]
    fn part_1_test() {
        assert_eq!(
            Some(part_1(&parse_ranges(util::input(DAY)))),
            util::answer(DAY, 1)
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            Some(part_2(&parse_ranges(util::input(DAY)))),
            util::answer(DAY, 2)
        );
    }
}
