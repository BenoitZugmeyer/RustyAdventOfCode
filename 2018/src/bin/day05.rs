use std::collections::HashSet;
use std::io::{stdin, BufRead};

fn are_reacting(a: char, b: char) -> bool {
    a.is_ascii_uppercase() && b == a.to_ascii_lowercase()
        || a.is_ascii_lowercase() && b == a.to_ascii_uppercase()
}

fn react_polymer_size<T>(input: T) -> usize
where
    T: Iterator<Item = char>,
{
    let mut mutated_polymer = Vec::new();

    for a in input {
        if let Some(b) = mutated_polymer.last() {
            if are_reacting(a, *b) {
                mutated_polymer.pop();
            } else {
                mutated_polymer.push(a);
            }
        } else {
            mutated_polymer.push(a);
        }
    }

    mutated_polymer.len()
}

fn main() {
    let input = stdin()
        .lock()
        .lines()
        .next()
        .expect("No line in input")
        .unwrap();

    println!("Part 1: {}", react_polymer_size(input.chars()));

    let all_units: HashSet<_> = input.chars().map(|ch| ch.to_ascii_lowercase()).collect();

    let shortest = all_units
        .iter()
        .map(|unit| {
            react_polymer_size(
                input
                    .chars()
                    .filter(|ch| ch != unit && *ch != unit.to_ascii_uppercase()),
            )
        })
        .min()
        .unwrap();
    println!("Part 2: {}", shortest);
}
