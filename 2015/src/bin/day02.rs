#![feature(slice_patterns)]
#![feature(iter_arith)]
use std::io;
use std::io::BufRead;

fn compute_surface_and_length(line: &str) -> (i32, i32) {
    let mut numbers = line.split('x').map(|n| n.parse::<i32>().unwrap_or(0)).collect::<Vec<i32>>();
    numbers.sort();

    let surface = match &numbers[..] {
        &[l, w, h] => {
            let surfaces = vec![ l * w, l * h, w * h ];
            surfaces.iter().sum::<i32>() * 2 + surfaces.iter().min().unwrap()
        }
        _ => 0,
    };

    let length = numbers.iter().product::<i32>() + numbers.iter().take(2).sum::<i32>() * 2;

    (surface, length)
}

fn main() {
    let stdin = io::stdin();

    let (surface, length) = stdin.lock().lines()
        .filter_map(|line| line.ok())
        .fold((0, 0), |(total_surface, total_length), ref line| {
            let (surface, length) = compute_surface_and_length(line);
            (total_surface + surface, total_length + length)
        });

    println!("Total surface: {}", surface);
    println!("Total length: {}", length);

}

