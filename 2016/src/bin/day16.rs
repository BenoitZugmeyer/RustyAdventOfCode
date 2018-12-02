extern crate itertools;
use itertools::Itertools;
use std::io::stdin;
use std::io::Read;

fn fill_until(vec: &mut Vec<bool>, len: usize) {
    vec.reserve(len);
    while vec.len() < len {
        vec.push(false);
        if vec.len() == len {
            return;
        }
        for index in (0..vec.len() - 1).rev() {
            let v = !vec[index];
            vec.push(v);
            if vec.len() == len {
                return;
            }
        }
    }
}

fn checksum(vec: &[bool]) -> Vec<bool> {
    let mut current: Vec<_> = vec.iter().tuples().map(|(a, b)| a == b).collect();
    while current.len() % 2 != 1 {
        current = current.iter().tuples().map(|(a, b)| a == b).collect();
    }
    current
}

fn print(vec: &[bool]) {
    for v in vec {
        if *v {
            print!("1");
        } else {
            print!("0");
        }
    }
    println!();
}

fn main() {
    let start: Vec<_> = stdin()
        .bytes()
        .filter_map(|b| b.ok())
        .filter_map(|b| match b {
            b'1' => Some(true),
            b'0' => Some(false),
            _ => None,
        })
        .collect();

    let mut disk1 = start.clone();
    fill_until(&mut disk1, 272);
    print!("Part 1: ");
    print(&checksum(&disk1));

    let mut disk2 = start.clone();
    fill_until(&mut disk2, 35_651_584);
    print!("Part 2: ");
    print(&checksum(&disk2));
}
