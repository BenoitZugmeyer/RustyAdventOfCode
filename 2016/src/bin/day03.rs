extern crate itertools;
use itertools::Itertools;
use std::io::stdin;
use std::io::Read;

fn is_valid(mut lengths: Vec<u32>) -> usize {
    lengths.sort();
    if lengths[0] + lengths[1] > lengths[2] { 1 } else { 0 }
}

fn main() {

    let (row_count, column_count) = stdin()
        .bytes()
        .filter_map(|b| b.ok())
        .scan(0, |state, ch| {
            Some(if ch == b' ' || ch == b'\n' {
                if *state > 0 {
                    let res = *state;
                    *state = 0;
                    Some(res)
                } else {
                    None
                }
            } else {
                *state = *state * 10 + u32::from(ch - b'0');
                None
            })
        })
        .filter_map(|b| b)
        .chunks(9)
        .into_iter()
        .fold((0, 0), |(row_count, column_count), lengths_iter| {
            let lengths = lengths_iter.collect::<Vec<_>>();

            let new_row_count = is_valid(lengths[0..3].to_vec()) +
                                is_valid(lengths[3..6].to_vec()) +
                                is_valid(lengths[6..9].to_vec());

            let new_column_count = is_valid(vec![lengths[0], lengths[3], lengths[6]]) +
                                   is_valid(vec![lengths[1], lengths[4], lengths[7]]) +
                                   is_valid(vec![lengths[2], lengths[5], lengths[8]]);

            (row_count + new_row_count, column_count + new_column_count)
        });

    println!("Part 1: {}", row_count);
    println!("Part 2: {}", column_count);
}
