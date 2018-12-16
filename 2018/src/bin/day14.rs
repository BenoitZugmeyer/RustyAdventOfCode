use std::io::{stdin, BufRead};

struct Board {
    scores: Vec<u8>,
    positions: [usize; 2],
    reminder: Option<u8>,
}

impl Board {
    fn new() -> Self {
        Self {
            scores: vec![3, 7],
            positions: [0, 1],
            reminder: None,
        }
    }

    fn update_positions(&mut self) {
        for position in &mut self.positions {
            *position = (*position + self.scores[*position] as usize + 1) % self.scores.len();
        }
    }

    fn next(&mut self) {
        if let Some(reminder) = self.reminder.take() {
            self.scores.push(reminder);
            self.update_positions()
        } else {
            let sum = self.positions.iter().map(|&p| self.scores[p]).sum::<u8>();
            if sum >= 10 {
                self.scores.push(sum / 10);
                self.reminder = Some(sum % 10);
            } else {
                self.scores.push(sum % 10);
                self.update_positions()
            }
        }
    }
}

fn part_1(number_of_recipes: usize) -> u64 {
    let mut board = Board::new();
    for _ in 0..number_of_recipes + 10 {
        board.next();
    }
    board.scores[number_of_recipes..number_of_recipes + 10]
        .iter()
        .fold(0, |digits, &score| digits * 10 + u64::from(score))
}

fn part_2(digits: &[u8]) -> u64 {
    let mut board = Board::new();

    loop {
        let len = board.scores.len().saturating_sub(digits.len());
        if &board.scores[len..] == digits {
            return len as u64;
        }

        board.next();
    }
}

fn main() {
    let input = stdin()
        .lock()
        .lines()
        .filter_map(|l| l.ok())
        .next()
        .expect("Failed to read input line");

    println!(
        "Part 1: {:?}",
        part_1(input.parse().unwrap())
    );

    println!(
        "Part 2: {:?}",
        part_2(&input.chars().map(|ch| ch as u8 - b'0').collect::<Vec<_>>())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(9), 5_158_916_779);
        assert_eq!(part_1(5), 124_515_891);
        assert_eq!(part_1(18), 9_251_071_085);
        assert_eq!(part_1(2018), 5_941_429_882);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&[5, 1, 5, 8, 9]), 9);
        assert_eq!(part_2(&[0, 1, 2, 4, 5]), 5);
        assert_eq!(part_2(&[9, 2, 5, 1, 0]), 18);
        assert_eq!(part_2(&[5, 9, 4, 1, 4]), 2018);
    }
}
