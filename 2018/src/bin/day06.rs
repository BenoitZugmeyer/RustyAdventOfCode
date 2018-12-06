use itertools::Itertools;
use std::io::{stdin, BufRead};

fn main() {
    let coordinates: Vec<(i32, i32)> = stdin()
        .lock()
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| {
            let mut values = l.split(", ");
            Some((values.next()?.parse().ok()?, values.next()?.parse().ok()?))
        })
        .collect();

    let (min_x, max_x) = coordinates
        .iter()
        .map(|(x, _)| *x)
        .minmax()
        .into_option()
        .unwrap();

    let (min_y, max_y) = coordinates
        .iter()
        .map(|(_, y)| *y)
        .minmax()
        .into_option()
        .unwrap();

    let mut distances = vec![Some(0); coordinates.len()];
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let mut min_coordinate = None;
            let mut min_distance = 1_000_000;
            for (index, coordinate) in coordinates.iter().enumerate() {
                let distance = i32::abs(coordinate.0 - x) + i32::abs(coordinate.1 - y);
                if distance == min_distance {
                    // Two coordinates have the same distance, discard the coordinate
                    min_coordinate = None;
                } else if distance < min_distance {
                    min_coordinate = Some(index);
                    min_distance = distance;
                }
            }
            if let Some(index) = min_coordinate {
                if x == max_x || y == max_y {
                    distances[index] = None;
                } else if let Some(d) = distances[index].as_mut() {
                    *d += 1;
                }
            }
        }
    }

    let max = distances.iter().filter_map(|d| *d).max().unwrap();

    println!("Part 1: {:?}", max);

    let mut safe_region_size = 0;

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let distances_sum = coordinates
                .iter()
                .map(|coordinate| i32::abs(coordinate.0 - x) + i32::abs(coordinate.1 - y))
                .sum::<i32>();
            if distances_sum < 10_000 {
                safe_region_size += 1;
            }
        }
    }
    println!("Part 2: {}", safe_region_size);
}
