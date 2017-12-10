extern crate itertools;

use std::io::stdin;
use std::io::Read;
use itertools::Itertools;

fn rotate_list(lengths: &[usize], rounds: u16) -> Vec<u8> {
    let mut position: usize = 0;
    let mut skip_size: usize = 0;

    let mut list: Vec<_> = (0..256u16).map(|n| n as u8).collect();

    let list_len = list.len();
    for _ in 0..rounds {
        for length in lengths.iter() {
            list = list.iter()
                .enumerate()
                .map(|(index, value)| {
                    let relative_index = (list_len + index - position) % list_len;
                    if relative_index < *length {
                        list[(position + *length - relative_index - 1) % list_len]
                    } else {
                        *value
                    }
                })
                .collect();

            position = (position + length + skip_size) % list_len;
            skip_size += 1;
        }
    }

    list
}

fn part_1(input: &str) -> u16 {
    let lengths: Vec<usize> = input
        .split(',')
        .flat_map(|n| n.trim().parse().ok())
        .collect();

    let list = rotate_list(&lengths, 1);
    u16::from(list[0]) * u16::from(list[1])
}

fn part_2(input: &str) -> String {
    let mut lengths: Vec<_> = input.trim().chars().map(|ch| ch as usize).collect();
    lengths.extend(&[17, 31, 73, 47, 23]);
    let sparse_hash = rotate_list(&lengths, 64);
    let dense_hash: Vec<_> = sparse_hash.into_iter()
        .chunks(16)
        .into_iter()
        .map(|chunk| chunk.fold1(|r, n| r ^ n).unwrap())
        .collect();
    dense_hash.iter().map(|n| format!("{:02x}", n)).collect()
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).expect(
        "Failed to read stdin",
    );

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
