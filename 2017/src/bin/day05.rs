use std::io::stdin;
use std::io::BufRead;

fn part_1(mut map: Vec<i32>) -> u32 {
    let mut position: i32 = 0;
    let mut move_count = 0;
    loop {
        let value = map[position as usize];
        map[position as usize] += 1;
        position += value;
        move_count += 1;
        if position < 0 || position >= map.len() as i32 {
            break;
        }
    }
    move_count
}

fn part_2(mut map: Vec<i32>) -> u32 {
    let mut position: i32 = 0;
    let mut move_count = 0;
    let len = map.len() as i32;
    // while let Some(value) = map.get_mut(position as usize) {
    //     position += *value;
    //     *value += if *value >= 3 { -1 } else { 1 };
    //     move_count += 1;
    // }
    loop {
        unsafe {
            let value = map.get_unchecked_mut(position as usize);
            position += *value;
            *value += if *value >= 3 { -1 } else { 1 };
        }
        move_count += 1;
        if position < 0 || position >= len {
            break;
        }
    }
    move_count
}

fn main() {
    let stdin = stdin();
    let map: Vec<i32> = stdin
        .lock()
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|line| line.parse().ok())
        .collect();

    println!("Part 1: {}", part_1(map.clone()));
    println!("Part 2: {}", part_2(map));
}
