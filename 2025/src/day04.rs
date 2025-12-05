use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Roll,
}

type Grid = Vec<Vec<Cell>>;

fn parse_grid(input: impl Iterator<Item = String>) -> Grid {
    input
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Cell::Empty,
                    '@' => Cell::Roll,
                    _ => panic!("Unexpected char {}", c),
                })
                .collect()
        })
        .collect()
}

fn part_1(grid: Grid) -> usize {
    let grid_width = grid[0].len();
    let grid_height = grid.len();

    iter_coords(0..grid_width, 0..grid_height)
        .filter(|(x, y)| grid[*y][*x] != Cell::Empty && can_remove(&grid, *x, *y))
        .count()
}

fn part_2(mut grid: Grid) -> usize {
    let grid_width = grid[0].len();
    let grid_height = grid.len();
    let mut removed_rolls = 0;
    loop {
        let mut removed_something = false;
        let mut next_grid = grid.clone();

        for (x, y) in iter_coords(0..grid_width, 0..grid_height) {
            if grid[y][x] == Cell::Roll && can_remove(&grid, x, y) {
                next_grid[y][x] = Cell::Empty;
                removed_something = true;
                removed_rolls += 1;
            }
        }
        if !removed_something {
            break;
        }
        grid = next_grid;
    }

    removed_rolls
}

fn iter_coords(xrange: Range<usize>, yrange: Range<usize>) -> impl Iterator<Item = (usize, usize)> {
    yrange.flat_map(move |y| xrange.clone().map(move |x| (x, y)))
}

fn can_remove(grid: &Grid, x: usize, y: usize) -> bool {
    let width = grid[0].len();
    let height = grid.len();
    let occupied_space = iter_coords(
        x.saturating_sub(1)..(x + 2).min(width),
        y.saturating_sub(1)..(y + 2).min(height),
    )
    .filter(|(dx, dy)| grid[*dy][*dx] == Cell::Roll)
    .count();
    occupied_space < 5
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;
    const DAY: u8 = 4;

    #[test]
    fn test1() {
        assert_eq!(part_1(parse_grid(util::example(DAY, 1))), 13);
    }

    #[test]
    fn test2() {
        assert_eq!(part_2(parse_grid(util::example(DAY, 1))), 43);
    }

    #[test]
    fn part_1_test() {
        assert_eq!(
            Some(part_1(parse_grid(util::input(DAY)))),
            util::answer(DAY, 1)
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            Some(part_2(parse_grid(util::input(DAY)))),
            util::answer(DAY, 2)
        );
    }
}
