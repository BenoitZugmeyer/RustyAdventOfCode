use itertools::Itertools;
use std::io::{stdin, BufRead};
use std::collections::HashMap;

fn main() {
    let box_ids: Vec<String> = stdin().lock().lines().filter_map(|l| l.ok()).collect();

    let mut two_letters = 0;
    let mut three_letters = 0;
    for box_id in box_ids.iter() {
        let mut letter_counts = HashMap::new();
        for letter in box_id.chars() {
            *letter_counts.entry(letter).or_insert(0) += 1
        }
        if letter_counts.values().any(|&count| count == 2) {
            two_letters += 1;
        }
        if letter_counts.values().any(|&count| count == 3) {
            three_letters += 1;
        }
    }
    println!("Part 1: {}", two_letters * three_letters);

    let mut common_letters: Option<String> = None;
    for (id1, id2) in box_ids.iter().tuple_combinations() {
        let are_close = id1.chars()
            .zip(id2.chars())
            .scan(0, |total, (l1, l2)| {
                if l1 != l2 {
                    *total += 1
                }
                Some(*total)
            })
            .all(|d| d <= 1);
        if are_close {
            common_letters = Some(
                id1.chars()
                    .zip(id2.chars())
                    .filter(|(l1, l2)| l1 == l2)
                    .map(|(l1, _)| l1)
                    .collect(),
            );
            break;
        }
    }
    println!(
        "Part 2: {}",
        common_letters.unwrap_or_else(|| "Not found".to_string())
    );
}
