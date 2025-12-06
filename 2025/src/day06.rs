fn part_1(lines: impl Iterator<Item = String>) -> u64 {
    let mut rows: Vec<Vec<u64>> = Vec::new();
    let mut operands: Vec<char> = Vec::new();

    for line in lines {
        let mut row = Vec::new();
        for n in line.split(' ') {
            if let Ok(n) = n.parse() {
                row.push(n);
            }
        }
        if !row.is_empty() {
            rows.push(row);
        } else {
            for ch in line.chars() {
                match ch {
                    '*' | '+' => operands.push(ch),
                    _ => {}
                };
            }
        }
    }

    operands
        .iter()
        .enumerate()
        .map(|(i, op)| compute(*op, rows.iter().map(|row| row[i])))
        .sum()
}

fn part_2(lines: impl Iterator<Item = String>) -> u64 {
    let mut numbers: Vec<Option<u64>> = Vec::new();

    for line in lines {
        if line.starts_with('+') || line.starts_with('*') {
            return line
                .chars()
                .enumerate()
                .filter(|(_, op)| *op != ' ')
                .map(|(i, op)| compute(op, numbers.iter().skip(i).map_while(|n| *n)))
                .sum();
        } else {
            for (i, ch) in line.chars().enumerate() {
                if let Some(d) = ch.to_digit(10) {
                    let d = d as u64;
                    if numbers.len() <= i {
                        numbers.resize(i + 1, None);
                    }
                    numbers[i] = Some(numbers[i].map(|n| n * 10 + d).unwrap_or(d));
                }
            }
        }
    }

    unreachable!()
}

fn compute(op: char, numbers_it: impl Iterator<Item = u64>) -> u64 {
    if op == '+' {
        numbers_it.sum::<u64>()
    } else {
        numbers_it.product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;
    const DAY: u8 = 6;

    #[test]
    fn test1() {
        assert_eq!(part_1(util::example(DAY, 1)), 4277556);
    }

    #[test]
    fn test2() {
        assert_eq!(part_2(util::example(DAY, 1)), 3263827);
    }

    #[test]
    fn part_1_test() {
        assert_eq!(Some(part_1(util::input(DAY))), util::answer(DAY, 1));
    }

    #[test]
    fn part_2_test() {
        assert_eq!(Some(part_2(util::input(DAY))), util::answer(DAY, 2));
    }
}
