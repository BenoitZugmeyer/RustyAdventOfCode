use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::io::{stdin, BufRead};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Square {
    Sand,
    Clay,
    StaleWater,
    DrippingWater,
    OutOfReach,
}

struct Grid {
    grid: Vec<Vec<Square>>,
    source: (usize, usize),
}

impl Grid {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([xy])=(\d+), [xy]=(\d+)\.\.(\d+)").unwrap();
        }

        let mut coords = Vec::new();

        for line in lines {
            if let Some(caps) = RE.captures(&line) {
                let vertical = caps.get(1).unwrap().as_str() == "x";
                let pos: usize = caps.get(2).unwrap().as_str().parse().unwrap();
                let from: usize = caps.get(3).unwrap().as_str().parse().unwrap();
                let to: usize = caps.get(4).unwrap().as_str().parse().unwrap();

                for i in from..=to {
                    if vertical {
                        coords.push((pos, i));
                    } else {
                        coords.push((i, pos));
                    }
                }
            }
        }

        let (min_x, max_x) = coords
            .iter()
            .map(|(x, _)| *x)
            .minmax()
            .into_option()
            .unwrap();

        let (min_y, max_y) = coords
            .iter()
            .map(|(_, y)| *y)
            .minmax()
            .into_option()
            .unwrap();

        let mut grid = vec![vec![Square::Sand; max_x - min_x + 3]; max_y - min_y + 1];
        for (x, y) in coords {
            grid[y - min_y][x - min_x + 1] = Square::Clay;
        }

        Self {
            grid,
            source: (
                500_usize.checked_sub(min_x).unwrap() + 1,
                0_usize.saturating_sub(min_y),
            ),
        }
    }

    fn at(&self, x: usize, y: usize) -> Square {
        self.grid
            .get(y)
            .and_then(|row| row.get(x))
            .cloned()
            .unwrap_or(Square::OutOfReach)
    }

    #[allow(dead_code)]
    fn print(&self) {
        for row in &self.grid {
            for square in row {
                print!(
                    "{}",
                    match square {
                        Square::Sand => '.',
                        Square::Clay => '#',
                        Square::StaleWater => '~',
                        Square::DrippingWater => '|',
                        Square::OutOfReach => 'X',
                    }
                );
            }
            println!();
        }
    }

    fn run(&mut self) {
        self.fill_from_source(self.source.0, self.source.1);
    }

    fn fill_from_source(&mut self, x: usize, mut y: usize) {
        loop {
            match self.at(x, y) {
                // We found the bottom, continue to fill the bowl
                Square::Clay | Square::StaleWater => break,

                // There is no bottom, stop here
                Square::OutOfReach | Square::DrippingWater => return,

                // We found some sand, continue dripping
                Square::Sand => {
                    self.grid[y][x] = Square::DrippingWater;
                    y += 1;
                }
            }
        }

        loop {
            // Gradually fill the bowl
            y -= 1;

            let (is_dripping_left, max_x) = self.is_driping(x - 1, y, true);
            let (is_driping_right, min_x) = self.is_driping(x, y, false);

            // Spread the water at the bottom
            for x in min_x..=max_x {
                self.grid[y][x] = if is_dripping_left || is_driping_right {
                    Square::DrippingWater
                } else {
                    Square::StaleWater
                };
            }

            if is_dripping_left || is_driping_right {
                if is_dripping_left {
                    self.fill_from_source(max_x, y + 1);
                }
                if is_driping_right {
                    self.fill_from_source(min_x, y + 1);
                }
                break;
            }
        }
    }

    fn is_driping(&self, mut x: usize, y: usize, direction: bool) -> (bool, usize) {
        loop {
            let px = x;
            if direction {
                x += 1;
            } else {
                x -= 1;
            }

            return match self.at(x, y) {
                // Edge case, should not happen
                Square::OutOfReach => (true, px),

                // The next square is filled, the water is not dripping
                Square::StaleWater | Square::Clay => (false, px),

                // The next square is not filled, consider the square below
                Square::Sand | Square::DrippingWater => match self.at(x, y + 1) {
                    // The square below isn't filled, the water is dripping
                    Square::OutOfReach | Square::DrippingWater | Square::Sand => (true, x),

                    // The square below is filled, continue spreading the water
                    Square::StaleWater | Square::Clay => continue,
                },
            };
        }
    }

    fn count_water(&self) -> usize {
        self.grid
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&cell| cell == &Square::StaleWater || cell == &Square::DrippingWater)
            .count()
    }

    fn count_stale_water(&self) -> usize {
        self.grid
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&cell| cell == &Square::StaleWater)
            .count()
    }
}

fn main() {
    let mut grid = Grid::from_lines(stdin().lock().lines().filter_map(|l| l.ok()));
    grid.run();
    // grid.print();
    println!("Part 1: {}", grid.count_water());
    println!("Part 2: {}", grid.count_stale_water());
}
