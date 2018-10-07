extern crate itertools;

use std::io::stdin;
use std::io::BufRead;
use std::collections::HashSet;

fn is_valid_passphrase(line: &str) -> bool {
    let mut unique_words: HashSet<&str> = HashSet::new();
    for word in line.split(' ') {
        if unique_words.contains(word) {
            return false;
        }
        unique_words.insert(word);
    }
    true
}

fn is_valid_passphrase2(line: &str) -> bool {
    let mut unique_words: HashSet<Vec<char>> = HashSet::new();
    for word in line.split(' ') {
        let mut chars: Vec<_> = word.chars().collect();
        chars.sort();
        if unique_words.contains(&chars) {
            return false;
        }
        unique_words.insert(chars);
    }
    true
}

fn main() {
    let stdin = stdin();

    let counts = stdin.lock().lines().filter_map(|l| l.ok()).fold(
        (0, 0),
        |(count_1,
          count_2),
         line| {
            (
                count_1 + if is_valid_passphrase(&line) { 1 } else { 0 },
                count_2 + if is_valid_passphrase2(&line) { 1 } else { 0 },
            )
        },
    );
    println!("Part 1: {}", counts.0);
    println!("Part 2: {}", counts.1);
}
