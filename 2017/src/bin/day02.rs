extern crate itertools;

use std::io::stdin;
use std::io::BufRead;
use std::iter::repeat;
use itertools::Itertools;
use itertools::MinMaxResult;

macro_rules! product {
    ( $slice:expr ) => {
        $slice.iter().enumerate().flat_map(|(i, val)| {
            repeat(val).zip(
                $slice
                    .iter()
                    .enumerate()
                    .filter_map(move |(ib, valb)| {
                        if ib != i { Some(valb) } else { None }
                    })
            )
        })
    };
}

fn main() {
    let stdin = stdin();

    let checksum = stdin
        .lock()
        .lines()
        .filter_map(|l| l.ok())
        .map(|line| {
            line.split('\t')
                .flat_map(|s| s.parse::<u32>().ok())
                .collect::<Vec<_>>()
        })
        .fold((0, 0), |(total_1, total_2), row| {
            let minmax = row.iter().minmax();

            (
                total_1 +
                    if let MinMaxResult::MinMax(min, max) = minmax {
                        max - min
                    } else {
                        0
                    },
                total_2 +
                    if let Some((a, b)) = product!(row).find(|&(a, b)| a % b == 0) {
                        a / b
                    } else {
                        0
                    },
            )
        });

    println!("Part 1: {}", checksum.0);
    println!("Part 2: {}", checksum.1);
}
