use std::io::{stdin, BufRead};
use std::collections::HashMap;

type Cell = u32;

#[inline]
fn cell_size(cell: Cell) -> usize {
    if cell & (1 << 16) != 0 {
        4
    } else if cell & (1 << 9) != 0 {
        3
    } else {
        2
    }
}

fn cell_create(size: usize) -> Cell {
    1 << (size * size)
}

#[allow(dead_code)]
fn cell_print(cell: Cell) {
    let size = cell_size(cell);
    for i in 0..size {
        for j in 0..size {
            print!(
                "{}",
                if cell_get_bit(cell, i, j) == 0 {
                    '.'
                } else {
                    '#'
                }
            );
        }
        println!("")
    }
}

fn parse_pixels(s: &str) -> Cell {
    s.chars().fold(1, |result, ch| match ch {
        '.' => result << 1,
        '#' => result << 1 | 1,
        _ => result,
    })
}

#[inline]
fn cell_bit_pos(cell: Cell, row: usize, col: usize) -> usize {
    let size = cell_size(cell);
    size * size - (row * size + col) - 1
}

#[inline]
fn cell_get_bit(cell: Cell, row: usize, col: usize) -> u32 {
    (cell >> cell_bit_pos(cell, row, col)) & 1
}

#[inline]
fn cell_set_bit(cell: Cell, row: usize, col: usize, bit: u32) -> Cell {
    cell | (bit << cell_bit_pos(cell, row, col))
}

fn cell_transform<F>(cell: Cell, f: F) -> Cell
where
    F: Fn(usize, usize, usize) -> (usize, usize),
{
    let size = cell_size(cell);
    let mut result = cell_create(size);
    for i in 0..size {
        for j in 0..size {
            let (ni, nj) = f(size, i, j);
            result = cell_set_bit(result, ni, nj, cell_get_bit(cell, i, j));
        }
    }
    result
}

fn cell_rotate(cell: Cell) -> Cell {
    cell_transform(cell, |size, i, j| (j, size - i - 1))
}

fn cell_flip(cell: Cell) -> Cell {
    cell_transform(cell, |size, i, j| (size - i - 1, j))
}

fn cell_combinations(cell: Cell) -> [Cell; 8] {
    let fh = cell_flip(cell);
    [
        cell,
        cell_rotate(cell),
        cell_rotate(cell_rotate(cell)),
        cell_rotate(cell_rotate(cell_rotate(cell))),
        fh,
        cell_rotate(fh),
        cell_rotate(cell_rotate(fh)),
        cell_rotate(cell_rotate(cell_rotate(fh))),
    ]
}

struct Grid {
    cells: Vec<Cell>,
    size: usize,
    cell_size: usize,
    full_size: usize,
}

impl Grid {
    fn new(full_size: usize) -> Self {
        if full_size == 0 {
            panic!("Invalid full size");
        }

        let (size, cell_size) = if full_size % 2 == 0 {
            (full_size / 2, 2)
        } else {
            (full_size / 3, 3)
        };

        Grid {
            cells: vec![cell_create(cell_size); size * size],
            size,
            cell_size,
            full_size,
        }
    }

    #[inline]
    fn get_cell_index(&self, row: usize, col: usize) -> usize {
        let grid_row = row / self.cell_size;
        let grid_col = col / self.cell_size;
        grid_row * self.size + grid_col
    }

    #[inline]
    fn get_bit_index(&self, row: usize, col: usize) -> usize {
        let cell_size = self.cell_size;
        let cell_row = row % cell_size;
        let cell_col = col % cell_size;
        cell_size * cell_size - (cell_row * cell_size + cell_col) - 1
    }

    #[inline]
    fn get(&self, row: usize, col: usize) -> u32 {
        let index = self.get_cell_index(row, col);
        (self.cells[index] >> self.get_bit_index(row, col)) & 0b1
    }

    #[inline]
    fn set(&mut self, row: usize, col: usize, bit: u32) {
        let index = self.get_cell_index(row, col);
        self.cells[index] |= bit << self.get_bit_index(row, col);
    }

    fn set_cell(&mut self, grid_row: usize, grid_col: usize, cell: Cell) {
        let cell_size = cell_size(cell);

        if cell_size == self.cell_size {
            let index = self.get_cell_index(grid_row * cell_size, grid_col * cell_size);
            self.cells[index] = cell
        } else {
            for i in 0..cell_size {
                for j in 0..cell_size {
                    self.set(
                        grid_row * cell_size + i,
                        grid_col * cell_size + j,
                        cell_get_bit(cell, i, j),
                    );
                }
            }
        }
    }

    fn each_cell<F>(&self, mut f: F)
    where
        F: FnMut(Cell, usize, usize) -> (),
    {
        for i in 0..self.size {
            for j in 0..self.size {
                f(self.cells[i * self.size + j], i, j);
            }
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        let cell_size = self.cell_size;
        for row in 0..self.size * cell_size {
            for col in 0..self.size * cell_size {
                if self.get(row, col) != 0 {
                    print!("#");
                } else {
                    print!(".");
                }
                if col % cell_size == cell_size - 1 {
                    print!("|");
                }
            }
            println!("");
            if row % cell_size == cell_size - 1 {
                println!("-----");
            }
        }
    }
}

fn next_grid(grid: &Grid, transformations: &HashMap<Cell, Cell>) -> Grid {
    let mut result = Grid::new(if grid.full_size % 2 == 0 {
        grid.full_size * 3 / 2
    } else {
        grid.full_size * 4 / 3
    });
    grid.each_cell(|cell, i, j| result.set_cell(i, j, transformations[&cell]));
    result
}

fn main() {
    let stdin = stdin();

    let transformations: HashMap<Cell, Cell> = stdin
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
        .flat_map(|(source, target)| {
            cell_combinations(source)
                .iter()
                .map(|source| (*source, target))
                .collect::<Vec<(Cell, Cell)>>()
        })
        .collect();
    let mut grid = Grid::new(3);

    grid.set_cell(
        0,
        0,
        parse_pixels(
            "
                .#.
                ..#
                ###
            ",
        ),
    );

    for i in 0..18 {
        grid = next_grid(&grid, &transformations);
        if i == 4 {
            println!(
                "Part 1: {}",
                grid.cells.iter().map(|v| v.count_ones() - 1).sum::<u32>()
            );
        }
    }
    println!(
        "Part 2: {}",
        grid.cells.iter().map(|v| v.count_ones() - 1).sum::<u32>()
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
