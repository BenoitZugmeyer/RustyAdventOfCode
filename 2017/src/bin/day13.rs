use std::io::{stdin, BufRead};

#[inline]
fn is_at_top(range: u32, timestamp: u32) -> bool {
    timestamp % (range * 2 - 2) == 0
}

fn main() {
    let stdin = stdin();

    let layers: Vec<(u32, u32)> = stdin
        .lock()
        .lines()
        .filter_map(|l| l.ok())
        .map(|line| {
            let v: Vec<u32> = line.split(": ")
                .flat_map(|s| s.trim().parse().ok())
                .collect();
            (v[0], v[1])
        })
        .collect();

    let mut severity = 0;
    for &(depth, range) in &layers {
        if is_at_top(range, depth) {
            severity += depth * range;
        }
    }

    println!("Part 1: {}", severity);

    for delay in 0.. {
        let mut success = true;
        for &(depth, range) in &layers {
            if is_at_top(range, depth + delay) {
                success = false;
                break;
            }
        }
        if success {
            println!("Part 2: {}", delay);
            break;
        }
    }
}
