#![feature(test)]
extern crate test;

/// Solution using a `VecDeque` to represent the board, inspired by
/// [Frank de Jong solution](https://github.com/foo-jin/advent-of-code/blob/master/2018/day09/src/main.rs)
/// This is slightly more efficient than my solution using a double linked list.
use regex::Regex;
use std::collections::VecDeque;
use std::io::{stdin, BufRead};

fn max_score(player_count: usize, max_marble: usize) -> usize {
    let mut board = VecDeque::with_capacity(max_marble);
    board.push_front(0);
    let mut scores = vec![0; player_count];

    for marble in 1..=max_marble {
        let player = (marble - 1) % player_count;
        if marble % 23 == 0 {
            // Move backward 7 times, compute the score and remove the marble
            for _ in 0..7 {
                let value = board.pop_back().unwrap();
                board.push_front(value);
            }
            scores[player] += marble + board.pop_front().unwrap();
        } else {
            // Move forward two times and insert the marble into the board
            for _ in 0..2 {
                let value = board.pop_front().unwrap();
                board.push_back(value);
            }
            board.push_front(marble);
        }
    }

    *scores.iter().max().unwrap()
}

fn main() {
    let line = stdin()
        .lock()
        .lines()
        .filter_map(|l| l.ok())
        .next()
        .unwrap_or_else(String::new);

    let re = Regex::new(r"\d+").unwrap();
    let mut iter = re
        .captures_iter(&line)
        .filter_map(|capture| capture.get(0)?.as_str().parse::<usize>().ok());
    let player_count = iter.next().unwrap();
    let max_marble = iter.next().unwrap();

    println!("Part 1: {}", max_score(player_count, max_marble));
    println!("Part 2: {}", max_score(player_count, max_marble * 100));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| max_score(468, 71010 * 100))
    }
}
