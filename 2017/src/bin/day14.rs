extern crate aoc2017;
use std::io::{stdin, Read};
use aoc2017::knot;

fn is_occupied(grid: &[Vec<u8>], row: usize, col: usize) -> bool {
    let byte = grid[row][col / 8];
    byte & (0x80 >> (col % 8)) != 0
}

fn delete_group(mut grid: &mut [Vec<u8>], row: usize, col: usize) {
    grid[row][col / 8] &= !(0x80 >> (col % 8));
    if col > 0 && is_occupied(grid, row, col - 1) {
        delete_group(&mut grid, row, col - 1);
    }
    if row > 0 && is_occupied(grid, row - 1, col) {
        delete_group(&mut grid, row - 1, col);
    }
    if col < grid[row].len() * 8 - 1 && is_occupied(grid, row, col + 1) {
        delete_group(&mut grid, row, col + 1);
    }
    if row < grid.len() - 1 && is_occupied(grid, row + 1, col) {
        delete_group(&mut grid, row + 1, col);
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).expect(
        "Failed to read stdin",
    );
    input = input.trim().to_string();

    let mut grid: Vec<Vec<u8>> = (0..128)
        .map(|i| knot::compute_hash_bytes(&format!("{}-{}", input, i)))
        .collect();
    let part_1 = grid.iter()
        .map(|row| row.iter().map(|byte| byte.count_ones()).sum::<u32>())
        .sum::<u32>();
    println!("Part 1: {}", part_1);

    let mut count = 0;
    for row_index in 0..128 {
        for col_index in 0..128 {
            if is_occupied(&grid, row_index, col_index) {
                delete_group(&mut grid, row_index, col_index);
                count += 1;
            }
        }
    }
    println!("Part 2: {}", count);
}
