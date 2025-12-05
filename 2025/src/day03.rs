#[allow(dead_code)]
fn part_1(banks: impl Iterator<Item = Vec<u8>>) -> u64 {
    let mut sum = 0;
    for bank in banks {
        sum += find_biggest_joltage(&bank, 2);
    }
    sum
}

#[allow(dead_code)]
fn part_2(banks: impl Iterator<Item = Vec<u8>>) -> u64 {
    let mut sum = 0;
    for bank in banks {
        sum += find_biggest_joltage(&bank, 12);
    }
    sum
}

fn find_biggest_joltage(bank: &[u8], digits: usize) -> u64 {
    let mut sum = 0;

    let mut index = 0;
    for d in 1..=digits {
        let (i, b) = find_biggest_bank(&bank[index..bank.len() - (digits - d)]);
        sum += (b as u64) * 10u64.pow((digits - d) as u32);
        index += i + 1;
    }

    sum
}

fn find_biggest_bank(bank: &[u8]) -> (usize, u8) {
    let mut result = (0, 0);
    for (i, b) in bank.iter().enumerate() {
        if *b > result.1 {
            result = (i, *b);
        }
    }
    result
}

#[allow(dead_code)]
fn parse_bank(line: String) -> Vec<u8> {
    line.chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;
    const DAY: u8 = 3;

    #[test]
    fn test1() {
        assert_eq!(part_1(util::example(DAY, 1).map(parse_bank)), 357);
    }

    #[test]
    fn test2() {
        assert_eq!(part_2(util::example(DAY, 1).map(parse_bank)), 3121910778619);
    }

    #[test]
    fn part_1_test() {
        assert_eq!(
            Some(part_1(util::input(DAY).map(parse_bank))),
            util::answer(DAY, 1)
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            Some(part_2(util::input(DAY).map(parse_bank))),
            util::answer(DAY, 2)
        );
    }
}
