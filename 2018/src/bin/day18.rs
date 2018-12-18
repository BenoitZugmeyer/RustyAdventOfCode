use itertools::iproduct;
use std::collections::HashMap;
use std::io::{stdin, BufRead};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Acre {
    Open,
    Trees,
    Lumberyard,
}

#[allow(dead_code)]
fn grid_print(grid: &[Vec<Acre>]) {
    for row in grid {
        for acre in row {
            print!(
                "{}",
                match acre {
                    Acre::Open => '.',
                    Acre::Trees => '|',
                    Acre::Lumberyard => '#',
                }
            );
        }
        println!();
    }
}

fn acres_count(iter: impl Iterator<Item = Acre>) -> (usize, usize) {
    iter.fold((0, 0), |(total_lumberyard, total_trees), acre| match acre {
        Acre::Lumberyard => (total_lumberyard + 1, total_trees),
        Acre::Trees => (total_lumberyard, total_trees + 1),
        _ => (total_lumberyard, total_trees),
    })
}

fn grid_next(grid: &[Vec<Acre>]) -> Vec<Vec<Acre>> {
    (0..grid.len())
        .map(|y| {
            (0..grid[y].len())
                .map(|x| {
                    let (lumberyards, trees) = acres_count(
                        iproduct!(y.saturating_sub(1)..=y + 1, x.saturating_sub(1)..=x + 1)
                            .filter(|&(ay, ax)| ay != y || ax != x)
                            .filter_map(|(ay, ax)| grid.get(ay).and_then(|row| row.get(ax)))
                            .cloned(),
                    );
                    match grid[y][x] {
                        Acre::Open => {
                            if trees >= 3 {
                                Acre::Trees
                            } else {
                                Acre::Open
                            }
                        }
                        Acre::Trees => {
                            if lumberyards >= 3 {
                                Acre::Lumberyard
                            } else {
                                Acre::Trees
                            }
                        }
                        Acre::Lumberyard => {
                            if lumberyards >= 1 && trees >= 1 {
                                Acre::Lumberyard
                            } else {
                                Acre::Open
                            }
                        }
                    }
                })
                .collect()
        })
        .collect()
}

fn grid_count(grid: &[Vec<Acre>]) -> (usize, usize) {
    acres_count(grid.iter().flat_map(|row| row.iter()).cloned())
}

fn main() {
    let mut grid: Vec<Vec<_>> = stdin()
        .lock()
        .lines()
        .filter_map(|l| {
            l.ok().map(|l| {
                l.chars()
                    .map(|ch| match ch {
                        '.' => Acre::Open,
                        '|' => Acre::Trees,
                        '#' => Acre::Lumberyard,
                        _ => panic!("Unknown character {}", ch),
                    })
                    .collect()
            })
        })
        .collect();

    let mut map = HashMap::new();

    for minute in 0.. {
        // grid_print(&grid);
        let (trees_count, lumberyard_count) = grid_count(&grid);

        if minute == 10 {
            println!("Part 1: {}", trees_count * lumberyard_count);
        }

        // Collect counts into a map, to try to find a recurring period
        let entries = map
            .entry((trees_count, lumberyard_count))
            .or_insert_with(Vec::new);
        entries.push(minute);

        // If a given count tuple is repeating 5 times, we consider it as a period
        if entries.len() == 5 {
            let period = entries[4] - entries[3];
            let remaining = 1_000_000_000 - minute;
            for _ in 0..remaining % period {
                grid = grid_next(&grid);
            }
            let (trees_count, lumberyard_count) = grid_count(&grid);
            println!("Part 2: {}", trees_count * lumberyard_count);
            break;
        }

        grid = grid_next(&grid);
    }
}
