use std::io::stdin;
use std::io::BufRead;
use std::collections::VecDeque;

fn winner(elves: u32) -> u32 {
    if elves.is_power_of_two() {
        1
    } else {
        ((elves << 1) ^ elves.next_power_of_two()) | 1
    }
}

fn next_power_of_three(mut n: u32) -> u32 {
    let mut result = 1;
    while n > 0 {
        n /= 3;
        result *= 3;
    }
    result
}

fn winner2(elves: u32) -> u32 {
    let next = next_power_of_three(elves);
    let prev = next / 3;
    if elves == prev {
        elves
    } else if elves > prev * 2 {
        elves * 2 - next
    } else {
        elves - prev
    }
}

#[allow(dead_code)]
fn winner2_brute(elves: u32) -> u32 {
    let mut elves_vec: VecDeque<_> = (1..=elves).collect();
    let mut index = 0;

    while elves_vec.len() > 1 {
        let opposite = (index + elves_vec.len() / 2) % elves_vec.len();
        elves_vec.remove(opposite);
        index = if opposite > index {
            index + 1
        } else {
            index
        } % elves_vec.len();
    }

    elves_vec[0]
}

fn main() {
    let stdin = stdin();
    let elves: u32 = stdin.lock()
        .lines()
        .next()
        .and_then(|line| line.ok().and_then(|line| line.parse().ok()))
        .unwrap_or(0);

    println!("Part 1: {}", winner(elves));
    println!("Part 2: {}", winner2(elves));
}
