#[cfg(test)]
mod tests {
    use crate::util;
    use std::convert::TryFrom;

    #[derive(Debug, Copy, Clone)]
    enum Instruction {
        DealIntoNewDeck,
        Cut(i32),
        DealWithIncrement(usize),
    }

    impl std::str::FromStr for Instruction {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if s.starts_with("deal into") {
                Ok(Self::DealIntoNewDeck)
            } else if s.starts_with("cut ") {
                let n: i32 = s.split(' ').nth(1).unwrap().parse().unwrap();
                Ok(Self::Cut(n))
            } else if s.starts_with("deal with ") {
                let n: usize = s.split(' ').nth(3).unwrap().parse().unwrap();
                Ok(Self::DealWithIncrement(n))
            } else {
                Err(())
            }
        }
    }

    fn shuffle_deck<S: AsRef<str>, I: Iterator<Item = S>>(
        instructions: I,
        card_count: usize,
    ) -> Vec<usize> {
        let mut deck: Vec<_> = (0..card_count).collect();
        for instruction in instructions {
            match instruction.as_ref().parse().unwrap() {
                Instruction::DealIntoNewDeck => {
                    deck.reverse();
                }
                Instruction::Cut(n) => {
                    let n = usize::try_from(i32::try_from(card_count).unwrap() + n).unwrap()
                        % card_count;
                    let mut new_deck = deck[n..].to_vec();
                    new_deck.extend_from_slice(&deck[0..n]);
                    deck = new_deck;
                }
                Instruction::DealWithIncrement(n) => {
                    let mut new_deck = vec![0; card_count];
                    for (i, card) in deck.into_iter().enumerate() {
                        new_deck[(i * n) % card_count] = card;
                    }
                    deck = new_deck;
                }
            }
        }
        deck
    }

    fn get_card_position(instructions: &[Instruction], card_count: usize, card: usize) -> usize {
        let mut position = card;
        for instruction in instructions.iter().rev() {
            match instruction {
                Instruction::DealIntoNewDeck => {
                    position = card_count - position - 1;
                }
                Instruction::Cut(n) => {
                    let n = usize::try_from(i32::try_from(card_count).unwrap() + n).unwrap();
                    position = (position + n) % card_count;
                }
                Instruction::DealWithIncrement(n) => {
                    position = if position == 0 {
                        0
                    } else {
                        card_count - (position * n) % card_count
                    };
                }
            }
        }
        position
    }

    #[test]
    fn test_1_1() {
        assert_eq!(
            shuffle_deck("deal into new stack".split('\n'), 10),
            [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        );
        assert_eq!(
            shuffle_deck("cut 3".split('\n'), 10),
            [3, 4, 5, 6, 7, 8, 9, 0, 1, 2]
        );
        assert_eq!(
            shuffle_deck("cut -4".split('\n'), 10),
            [6, 7, 8, 9, 0, 1, 2, 3, 4, 5]
        );
        assert_eq!(
            shuffle_deck("deal with increment 3".split('\n'), 10),
            [0, 7, 4, 1, 8, 5, 2, 9, 6, 3]
        );
    }

    #[test]
    fn part_1() {
        assert_eq!(
            shuffle_deck(util::input(22), 10007)
                .into_iter()
                .position(|card| card == 2019),
            util::answer(22, 1)
        );
    }

    #[test]
    fn test_2_1() {
        assert_eq!(
            get_card_position(&[Instruction::DealWithIncrement(3)], 10, 0),
            0
        );
        assert_eq!(
            get_card_position(&[Instruction::DealWithIncrement(3)], 10, 1),
            7
        );
        assert_eq!(
            get_card_position(&[Instruction::DealWithIncrement(3)], 10, 2),
            4
        );
        assert_eq!(
            get_card_position(&[Instruction::DealWithIncrement(3)], 10, 3),
            1
        );
        assert_eq!(get_card_position(&[Instruction::Cut(3)], 10, 0), 3);
        assert_eq!(get_card_position(&[Instruction::Cut(3)], 10, 1), 4);
        assert_eq!(get_card_position(&[Instruction::Cut(3)], 10, 4), 7);
        assert_eq!(get_card_position(&[Instruction::Cut(3)], 10, 9), 2);

        assert_eq!(get_card_position(&[Instruction::DealIntoNewDeck], 10, 3), 6);
        assert_eq!(
            get_card_position(&[Instruction::DealWithIncrement(7)], 10, 6),
            8
        );
        assert_eq!(get_card_position(&[Instruction::Cut(6)], 10, 8), 4);

        assert_eq!(
            get_card_position(
                &[
                    Instruction::Cut(6),
                    Instruction::DealWithIncrement(7),
                    Instruction::DealIntoNewDeck
                ],
                10,
                3
            ),
            4
        );
    }

    #[test]
    #[ignore]
    fn part_2() {
        let _times = 101_741_582_076_661_u64;
        let _card_count = 119_315_717_514_047_u64;
        let _card = 2020;
        let _instructions: Vec<Instruction> =
            util::input(22).filter_map(|s| s.parse().ok()).collect();

        // I didn't find the solution for this part. I couldn't wrap my head around the maths
        // required to solve it.
    }
}
