use std::io::stdin;
use std::io::BufRead;
use std::collections::BTreeSet;


fn main() {
    let stdin = stdin();
    let blacklist: BTreeSet<(u32, u32)> = stdin.lock()
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|line| {
            let mut parts = line.split('-');
            if let (Some(low), Some(high)) = (parts.next(), parts.next()) {
                Some((low.parse().unwrap(), high.parse().unwrap()))
            } else {
                None
            }
        })
        .collect();

    let mut ip = 0;

    for &(low, high) in &blacklist {
        if ip < low {
            break;
        }
        ip = high + 1;
    }
    println!("Part 1: {}", ip);

    let mut count = 0;
    let mut max_high = 0;
    for &(low, high) in &blacklist {
        if low > max_high {
            count += low - max_high - 1;
        }
        max_high = std::cmp::max(high, max_high);
    }
    count += u32::max_value() - max_high;
    println!("Part 2: {}", count);
}
