#![feature(iter_arith)]
extern crate regex;
extern crate permutohedron;

use std::io;
use std::iter::once;
use std::io::BufRead;
use std::collections::BTreeMap;
use std::cmp::{max, min};
use regex::Regex;
use permutohedron::Heap;

fn push<T: Ord>(v: &mut Vec<T>, item: T) -> usize {
    if let Some(position) = v.iter().position(|other| *other == item) {
        position
    }
    else {
        v.push(item);
        v.len() - 1
    }
}

fn compute_key(index_a: usize, index_b: usize) -> (usize, usize) {
    (min(index_a, index_b), max(index_a, index_b))
}

fn compute_max_happiness<F: Fn(usize, usize) -> i32>(n: usize, getter: F) -> i32 {
    let mut indexes: Vec<_> = (1..n).collect();

    Heap::new(&mut indexes)
        .map(|sorted_indexes| {
            sorted_indexes.iter().chain(once(&0)).scan(0, |previous_index, current_index| {
                let result = getter(*previous_index, *current_index);
                *previous_index = *current_index;
                Some(result)
            })
            .sum::<i32>()
        })
        .max()
        .unwrap()
}



fn main() {

    let mut attendees: Vec<String> = Vec::new();
    let mut happiness: BTreeMap<(usize, usize), i32> = BTreeMap::new();

    let stdin = io::stdin();
    let re = Regex::new(r"(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+)").unwrap();

    for line in stdin.lock().lines().filter_map(|l| l.ok()) {
        if let Some(ref m) = re.captures(&line) {
            let index_a = push(&mut attendees, String::from(m.at(1).unwrap()));
            let index_b = push(&mut attendees, String::from(m.at(4).unwrap()));
            let key = compute_key(index_a, index_b);

            let h = {
                let h = m.at(3).unwrap().parse::<i32>().unwrap();
                if m.at(2).unwrap() == "lose" { -h } else { h }
            };

            *happiness.entry(key).or_insert(0) += h;
        }
    }

    let max = compute_max_happiness(attendees.len(), |index_a, index_b| {
        *happiness.get(&compute_key(index_a, index_b)).unwrap()
    });

    let max_with_me = compute_max_happiness(attendees.len() + 1, |index_a, index_b| {
        *happiness.get(&compute_key(index_a, index_b)).unwrap_or(&0)
    });

    println!("max: {}", max);
    println!("max with me: {}", max_with_me);
}
