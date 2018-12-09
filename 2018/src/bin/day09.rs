#![feature(test)]
extern crate test;

use regex::Regex;
use std::io::{stdin, BufRead};

#[derive(Debug)]
struct Node<T> {
    previous: usize,
    next: usize,
    value: T,
}

#[derive(Debug)]
struct Ring<T> {
    arena: Vec<Node<T>>,
    index: usize,
}

impl<T> Ring<T> {
    fn new(value: T) -> Self {
        Self {
            arena: vec![Node {
                previous: 0,
                next: 0,
                value,
            }],
            index: 0,
        }
    }

    fn value(&self) -> &T {
        &self.arena[self.index].value
    }

    fn next(&mut self) {
        self.index = self.arena[self.index].next;
    }

    fn previous(&mut self) {
        self.index = self.arena[self.index].previous;
    }

    fn insert(&mut self, value: T) {
        let ptr = self.arena.len();
        let previous = self.arena[self.index].previous;
        self.arena.push(Node {
            previous,
            next: self.index,
            value,
        });
        self.arena[self.index].previous = ptr;
        self.arena[previous].next = ptr;
        self.index = ptr;
    }

    fn remove(&mut self) {
        let Node { next, previous, .. } = self.arena[self.index];
        self.arena[next].previous = previous;
        self.arena[previous].next = next;
        self.index = next;
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Ring<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut index = self.index;
        loop {
            let node = &self.arena[index];
            write!(f, "{} ", node.value)?;
            index = node.next;
            if index == self.index {
                break;
            }
        }
        Ok(())
    }
}

fn max_score(player_count: usize, max_marble: usize) -> usize {
    let mut board = Ring::new(0);
    let mut scores = vec![0; player_count];

    for marble in 1..=max_marble {
        let player = (marble - 1) % player_count;
        if marble % 23 == 0 {
            // Move backward 7 times, compute the score and remove the marble
            for _ in 0..7 {
                board.previous();
            }
            scores[player] += marble + board.value();
            board.remove();
        } else {
            // Move forward two times and insert the marble into the board
            for _ in 0..2 {
                board.next();
            }
            board.insert(marble);
        }
        // if max_marble > 100 && marble % (max_marble / 100) == 0 {
        //     println!("{}", (marble as f64) / (max_marble as f64) * 100.);
        // }
        // println!("[{}] {}", player + 1, board);
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
