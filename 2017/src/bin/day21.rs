use std::io::{stdin, BufRead};
use std::collections::HashMap;

type Cell = u32;
type Grid = Vec<Cell>;

fn cell_size(cell: Cell) -> usize {
    if cell & (1 << 16) != 0 {
        4
    } else if cell & (1 << 9) != 0 {
        3
    } else {
        2
    }
}
fn grid_size(grid: &Grid) -> usize {
    (grid.len() as f32).sqrt() as usize
}

fn grid_get(grid: &Grid, row: usize, col: usize) -> u32 {
    let grid_size = grid_size(grid);
    let cell_size = cell_size(grid[0]);
    let grid_row = row / cell_size;
    let grid_col = col / cell_size;
    let cell_row = row % cell_size;
    let cell_col = col % cell_size;
    let cell = grid[grid_row * grid_size + grid_col];

    (cell >> (cell_size * cell_size - 1 - cell_row * cell_size - cell_col)) & 0b1
}

fn parse_pixels(s: &str) -> Cell {
    s.chars()
        .flat_map(|ch| match ch {
            '.' => Some(0),
            '#' => Some(1),
            _ => None,
        })
        .fold(1, |cell, value| (cell << 1) | value)
}

#[allow(dead_code)]
fn print_grid(grid: &Grid) {
    let grid_size = grid_size(grid);
    let cell_size = cell_size(grid[0]);
    for row in 0..grid_size * cell_size {
        for col in 0..grid_size * cell_size {
            if grid_get(grid, row, col) != 0 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn cell_rotate(cell: Cell) -> Cell {
    let size = cell_size(cell);

    #[cfg_attr(rustfmt, rustfmt_skip)]
    match size {
        2 => {
            0b1_00_00 |
                (cell & 0b10_00) >> 1 |
                (cell & 0b01_00) >> 2 |
                (cell & 0b00_10) << 2 |
                (cell & 0b00_01) << 1
        }
        3 => {
            0b1_000_000_000 |
                (cell & 0b100_001_000) >> 2 |
                (cell & 0b010_000_000) >> 4 |
                (cell & 0b001_000_000) >> 6 |
                (cell & 0b000_010_000) |
                (cell & 0b000_000_100) << 6 |
                (cell & 0b000_000_010) << 4 |
                (cell & 0b000_100_001) << 2
        }
        4 => {
            0b1_0000_0000_0000_0000 |
                (cell & 0b1000_0000_0001_0000) >> 3 |
                (cell & 0b0100_0001_0000_0000) >> 6 |
                (cell & 0b0010_0000_0000_0000) >> 9 |
                (cell & 0b0001_0000_0000_0000) >> 12 |
                (cell & 0b0000_0100_0000_0000) >> 1 |
                (cell & 0b0000_0010_0000_0000) >> 4 |
                (cell & 0b0000_0000_0100_0000) << 4 |
                (cell & 0b0000_0000_0010_0000) << 1 |
                (cell & 0b0000_0000_0001_0000) >> 3 |
                (cell & 0b0000_0000_0000_1000) << 12 |
                (cell & 0b0000_0000_0000_0100) << 9 |
                (cell & 0b0000_0000_1000_0010) << 6 |
                (cell & 0b0000_1000_0000_0001) << 3
        }
        _ => panic!("Unhandled size {}", size),
    }
}

fn cell_flip(cell: Cell) -> Cell {
    let size = cell_size(cell);

    #[cfg_attr(rustfmt, rustfmt_skip)]
    match size {
        2 => {
            0b1_00_00 |
                (cell & 0b11_00) >> 2 |
                (cell & 0b00_11) << 2
        },
        3 => {
            0b1_000_000_000 |
                (cell & 0b111_000_000) >> 6 |
                (cell & 0b000_111_000) |
                (cell & 0b000_000_111) << 6
        }
        4 => {
            0b1_0000_0000_0000_0000 |
                (cell & 0b1111_0000_0000_0000) >> 12 |
                (cell & 0b0000_1111_0000_0000) >> 4 |
                (cell & 0b0000_0000_1111_0000) << 4 |
                (cell & 0b0000_0000_0000_1111) << 12
        }
        _ => panic!("Unhandled size {}", size),
    }
}

fn find_transformation(mut cell: Cell, transformations: &HashMap<Cell, Cell>) -> Cell {
    {
        for _ in 0..2 {
            for _ in 0..4 {
                if let Some(transformed_cell) = transformations.get(&cell) {
                    return *transformed_cell;
                }

                cell = cell_rotate(cell);
            }
            cell = cell_flip(cell);
        }
    }

    panic!("Grid transformation not found");
}

fn next_grid(grid: &Grid, transformations: &HashMap<Cell, Cell>) -> Grid {
    let size = grid_size(grid) * cell_size(grid[0]);
    if size % 2 == 0 {
        let count = size / 2;
        (0..count)
            .flat_map(|mut orow| {
                orow *= 2;
                (0..count).map(move |mut ocol| {
                    ocol *= 2;
                    find_transformation(
                        0b1_00_00 | grid_get(grid, orow, ocol) << 3 |
                            grid_get(grid, orow, ocol + 1) << 2 |
                            grid_get(grid, orow + 1, ocol) << 1 |
                            grid_get(grid, orow + 1, ocol + 1),
                        transformations,
                    )
                })
            })
            .collect()
    } else {
        let count = size / 3;
        (0..count)
            .flat_map(|mut orow| {
                orow *= 3;
                (0..count).map(move |mut ocol| {
                    ocol *= 3;
                    find_transformation(
                        0b1_000_000_000 | grid_get(grid, orow, ocol) << 8 |
                            grid_get(grid, orow, ocol + 1) << 7 |
                            grid_get(grid, orow, ocol + 2) << 6 |
                            grid_get(grid, orow + 1, ocol) << 5 |
                            grid_get(grid, orow + 1, ocol + 1) << 4 |
                            grid_get(grid, orow + 1, ocol + 2) << 3 |
                            grid_get(grid, orow + 2, ocol) << 2 |
                            grid_get(grid, orow + 2, ocol + 1) << 1 |
                            grid_get(grid, orow + 2, ocol + 2),
                        transformations,
                    )
                })
            })
            .collect()
    }
}

fn main() {
    let stdin = stdin();

    let transformations: HashMap<_, _> = stdin
        .lock()
        .lines()
        .filter_map(|l| l.ok())
        .flat_map(|line| {
            line.find("=>").map(|position| {
                (
                    parse_pixels(&line[0..position]),
                    parse_pixels(&line[position..]),
                )
            })
        })
        .collect();
    let mut grid = vec![
        parse_pixels(
            "
                .#.
                ..#
                ###
            "
        ),
    ];

    for i in 0..18 {
        grid = next_grid(&grid, &transformations);
        if i == 4 {
            println!(
                "Part 1: {}",
                grid.iter().map(|v| v.count_ones() - 1).sum::<u32>()
            );
        }
    }
    println!(
        "Part 2: {}",
        grid.iter().map(|v| v.count_ones() - 1).sum::<u32>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cell_size_test() {
        assert_eq!(cell_size(0b1_00_00), 2);
        assert_eq!(cell_size(0b1_000_000_000), 3);
        assert_eq!(cell_size(0b1_0000_0000_0000_0000), 4);
    }

    #[test]
    fn cell_rotate_test() {
        assert_eq!(cell_rotate(0b1_10_10), 0b1_11_00);
        assert_eq!(cell_rotate(0b1_01_01), 0b1_00_11);
        assert_eq!(cell_rotate(0b1_11_11), 0b1_11_11);
        assert_eq!(cell_rotate(0b1_100_100_100), 0b1_111_000_000);
        assert_eq!(cell_rotate(0b1_111_111_111), 0b1_111_111_111);
        assert_eq!(
            cell_rotate(0b1_1000_1000_1000_1000),
            0b1_1111_0000_0000_0000
        );
        assert_eq!(
            cell_rotate(0b1_1111_1111_1111_1111),
            0b1_1111_1111_1111_1111
        );
    }

    #[test]
    fn cell_flip_test() {
        assert_eq!(cell_flip(0b1_11_00), 0b1_00_11);
        assert_eq!(cell_flip(0b1_10_10), 0b1_10_10);
        assert_eq!(cell_flip(0b1_111_000_000), 0b1_000_000_111);
        assert_eq!(cell_flip(0b1_1111_1001_0110_0000), 0b1_0000_0110_1001_1111);
    }
}
