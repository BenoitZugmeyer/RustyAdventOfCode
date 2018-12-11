use itertools::iproduct;
use rayon::prelude::*;
use std::io::{stdin, BufRead};

const GRID_SIZE: i32 = 300;

fn compute_power_level((x, y): (i32, i32), serial_number: i32) -> i32 {
    let rack_id = x + 10;
    (((rack_id * y + serial_number) * rack_id) / 100) % 10 - 5
}

fn compute_largest_total_power(serial_number: i32) -> (i32, i32) {
    let window = 3;
    iproduct!(1..=(GRID_SIZE - window), 1..=(GRID_SIZE - window))
        .max_by_key(|&(x, y)| {
            iproduct!(x..x + window, y..y + window)
                .map(|(x, y)| compute_power_level((x, y), serial_number))
                .sum::<i32>()
        })
        .unwrap()
}

fn compute_largest_total_power_for_cell(
    powers: &[i32],
    (x, y): (i32, i32),
) -> (i32, (i32, i32, i32)) {
    let mut max_power = 0;
    let mut result = (0, 0, 0);
    let mut power = 0;

    #[allow(clippy::cast_sign_loss)]
    for size in 0..GRID_SIZE - std::cmp::max(x, y) {
        power += (x..=x + size)
            .map(|x| powers[(x * GRID_SIZE + (y + size)) as usize])
            .sum::<i32>();
        power += (y..y + size)
            .map(|y| powers[((x + size) * GRID_SIZE + y) as usize])
            .sum::<i32>();

        if power > max_power {
            result = (x + 1, y + 1, size + 1);
            max_power = power;
        }
    }
    (max_power, result)
}

// /// Serial implementation
// fn compute_largest_total_power_any_size(serial_number: i32) -> (i32, i32, i32) {
//     let powers: Vec<_> = iproduct!(1..=GRID_SIZE, 1..=GRID_SIZE)
//         .map(|c| compute_power_level(c, serial_number))
//         .collect();

//     iproduct!(1..=(GRID_SIZE - 1), 1..=(GRID_SIZE - 1))
//         .map(|c| compute_largest_total_power_for_cell(&powers, c))
//         .max()
//         .map(|(_, result)| result)
//         .unwrap()
// }

/// Parallel implementation
fn compute_largest_total_power_any_size(serial_number: i32) -> (i32, i32, i32) {
    let powers: Vec<_> = iproduct!(1..=GRID_SIZE, 1..=GRID_SIZE)
        .map(|c| compute_power_level(c, serial_number))
        .collect();

    (0..GRID_SIZE)
        .into_par_iter()
        .map(|x| {
            (0..GRID_SIZE)
                .map(|y| compute_largest_total_power_for_cell(&powers, (x, y)))
                .max()
                .unwrap()
        })
        .max()
        .map(|(_, result)| result)
        .unwrap()
}

fn main() {
    let serial_number: i32 = stdin()
        .lock()
        .lines()
        .filter_map(|l| l.ok())
        .next()
        .expect("Failed to read input line")
        .parse()
        .expect("Failed to parse input line");

    println!("Part 1: {:?}", compute_largest_total_power(serial_number));

    println!(
        "Part 2: {:?}",
        compute_largest_total_power_any_size(serial_number)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_power_level() {
        assert_eq!(compute_power_level((3, 5), 8), 4);
        assert_eq!(compute_power_level((122, 79), 57), -5);
        assert_eq!(compute_power_level((217, 196), 39), 0);
        assert_eq!(compute_power_level((101, 153), 71), 4);
    }

    #[test]
    fn test_compute_largest_total_power() {
        assert_eq!(compute_largest_total_power(18), (33, 45));
        assert_eq!(compute_largest_total_power(42), (21, 61));
    }

    #[test]
    fn test_compute_largest_total_power_any_size() {
        assert_eq!(compute_largest_total_power_any_size(18), (90, 269, 16));
        assert_eq!(compute_largest_total_power_any_size(42), (232, 251, 12));
    }
}
