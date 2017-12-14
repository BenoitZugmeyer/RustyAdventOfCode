extern crate aoc2017;

use std::io::stdin;
use std::io::Read;
use aoc2017::knot;

fn part_1(input: &str) -> u16 {
    let lengths: Vec<usize> = input
        .split(',')
        .flat_map(|n| n.trim().parse().ok())
        .collect();

    let list = knot::rotate_list(&lengths, 1);
    u16::from(list[0]) * u16::from(list[1])
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).expect(
        "Failed to read stdin",
    );

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", knot::compute_hash(&input));
}
