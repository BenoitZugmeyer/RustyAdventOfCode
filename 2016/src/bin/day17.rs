extern crate itertools;
extern crate crypto;
use itertools::Itertools;
use itertools::MinMaxResult;
use std::io::stdin;
use std::io::Read;
use crypto::md5::Md5;
use crypto::digest::Digest;

type Coords = (u8, u8);

struct MinMaxIntoIter<T> {
    minmax: MinMaxResult<T>,
}

impl<T> Iterator for MinMaxIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match std::mem::replace(&mut self.minmax, MinMaxResult::NoElements) {
            MinMaxResult::OneElement(t) => Some(t),
            MinMaxResult::MinMax(min, max) => {
                self.minmax = MinMaxResult::OneElement(max);
                Some(min)
            }
            MinMaxResult::NoElements => None,
        }
    }
}

fn minmax_into_iter<T>(minmax: MinMaxResult<T>) -> MinMaxIntoIter<T> {
    MinMaxIntoIter { minmax }
}

fn open_doors(passcode: &[u8], (x, y): Coords, path: &[u8]) -> Vec<(u8, Coords)> {
    let mut result = Vec::new();
    let mut md5 = Md5::new();
    md5.input(passcode);
    md5.input(path);
    let hash = md5.result_str().into_bytes();
    if y > 0 && hash[0] > b'a' {
        result.push((b'U', (x, y - 1)));
    }
    if y < 3 && hash[1] > b'a' {
        result.push((b'D', (x, y + 1)));
    }
    if x > 0 && hash[2] > b'a' {
        result.push((b'L', (x - 1, y)));
    }
    if x < 3 && hash[3] > b'a' {
        result.push((b'R', (x + 1, y)));
    }
    result
}

fn find_path(passcode: &[u8], (x, y): Coords, mut path: &mut Vec<u8>) -> MinMaxResult<Vec<u8>> {
    if x == 3 && y == 3 {
        return MinMaxResult::OneElement(path.clone());
    }

    open_doors(passcode, (x, y), path)
        .iter()
        .flat_map(|&(direction, coords)| {
            path.push(direction);
            let result = find_path(passcode, coords, &mut path);
            path.pop();

            minmax_into_iter(result)
        })
        .minmax_by_key(|p| p.len())
}

fn main() {

    let passcode: Vec<_> = stdin()
        .bytes()
        .filter_map(|b| b.ok())
        .take_while(|b| b != &b'\n')
        .collect();

    if let MinMaxResult::MinMax(min, max) = find_path(&passcode, (0, 0), &mut Vec::new()) {
        println!("Part 1: {}", String::from_utf8_lossy(&min));
        println!("Part 2: {}", max.len());
    }
}
