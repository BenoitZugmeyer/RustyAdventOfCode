extern crate regex;
use regex::Regex;
use std::io::stdin;
use std::fmt;
use std::cmp::min;
use std::io::BufRead;


const ROWS: usize = 6;
const COLUMNS: usize = 50;
// const ROWS: usize = 3;
// const COLUMNS: usize = 7;

struct Screen {
    grid: [[bool; COLUMNS]; ROWS],
}

impl Screen {
    fn new() -> Self {
        Screen { grid: [[false; COLUMNS]; ROWS] }
    }
    fn rect(&mut self, a: usize, b: usize) {
        for x in 0..min(a, COLUMNS) {
            for y in 0..min(b, ROWS) {
                self.grid[y][x] = true;
            }
        }
    }
    fn rotate_row(&mut self, a: usize, b: usize) {
        let row = min(a, ROWS);
        let count = b % COLUMNS;
        let mut copy = [false; COLUMNS];
        copy.clone_from_slice(&self.grid[row]);

        for column in 0..COLUMNS {
            self.grid[row][column] = copy[(COLUMNS + column - count) % COLUMNS];
        }
    }
    fn rotate_column(&mut self, a: usize, b: usize) {
        let column = min(a, COLUMNS);
        let count = b % ROWS;
        let copy: Vec<_> = (0..ROWS).map(|row| self.grid[row][column]).collect();
        for row in 0..ROWS {
            self.grid[row][column] = copy[(ROWS + row - count) % ROWS];
        }
    }

    fn count(&self) -> usize {
        self.grid.iter().map(|row| row.iter().filter(|v| **v).count()).sum()
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for line in &self.grid {
            for v in line.iter() {
                if *v {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}


fn main() {
    let re = Regex::new(r"^(?x)(?:
        rect\s(\d+)x(\d+)
        |
        rotate\scolumn\sx=(\d+)\sby\s(\d+)
        |
        rotate\srow\sy=(\d+)\sby\s(\d+)
    )$")
        .unwrap();
    let stdin = stdin();
    let mut screen = Screen::new();
    for line in stdin.lock().lines().filter_map(|l| l.ok()) {
        if let Some(cap) = re.captures(&line) {
            if let (Some(a), Some(b)) = (cap.at(1), cap.at(2)) {
                screen.rect(a.parse().unwrap(), b.parse().unwrap());
            } else if let (Some(a), Some(b)) = (cap.at(3), cap.at(4)) {
                screen.rotate_column(a.parse().unwrap(), b.parse().unwrap());
            } else if let (Some(a), Some(b)) = (cap.at(5), cap.at(6)) {
                screen.rotate_row(a.parse().unwrap(), b.parse().unwrap());
            }
        }
    }
    println!("Part 1: {}", screen.count());
    println!("Part 2:\n{}", screen);
}
