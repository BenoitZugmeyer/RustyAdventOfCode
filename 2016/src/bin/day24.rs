extern crate itertools;
extern crate permutohedron;

use std::io::stdin;
use std::io::Read;
use std::collections::BTreeMap;
use permutohedron::LexicalPermutation;

use itertools::Itertools;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Copy)]
struct Coords {
    x: usize,
    y: usize,
}
impl Coords {
    fn new(x: usize, y: usize) -> Self {
        Coords { x, y }
    }
}

fn next(map: &[Vec<Cell>], coords: &Coords) -> Vec<Coords> {
    [(coords.x + 1, coords.y),
     (coords.x - 1, coords.y),
     (coords.x, coords.y + 1),
     (coords.x, coords.y - 1)]
        .iter()
        .map(|&(x, y)| Coords::new(x, y))
        .filter(|c| c.x > 0 && c.y > 0 && c.y < map.len() && c.x < map[c.y].len())
        .filter(|coord| map[coord.y][coord.x] == Cell::Path)
        .collect()
}

fn find_path_to(map: &[Vec<Cell>], position: &Coords, goal: &Coords) -> Option<u32> {
    let mut dmap = BTreeMap::new();
    dmap.insert(*position, 0);

    for i in 0.. {
        let aa: Vec<_> = dmap.iter().filter(|&(_, v)| v == &i).map(|(v, _)| *v).collect();
        let mut should_go_next = false;
        for p in &aa {
            for next in &next(map, p) {
                if next == goal {
                    return Some(i + 1);
                }
                if !dmap.contains_key(next) {
                    should_go_next = true;
                    dmap.insert(*next, i + 1);
                }
            }
        }

        if !should_go_next {
            return None;
        }
    }
    unreachable!();
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Copy)]
enum Cell {
    Wall,
    Path,
}

fn main() {
    let mut map = vec![vec![]];
    let mut poi = BTreeMap::new();
    for b in stdin().bytes().filter_map(|b| b.ok()) {
        match b {
            b'#' => map.last_mut().unwrap().push(Cell::Wall),
            b'.' => map.last_mut().unwrap().push(Cell::Path),
            b'0'...b'9' => {
                map.last_mut().unwrap().push(Cell::Path);
                poi.insert(b - b'0',
                           Coords::new(map.last().unwrap().len() - 1, map.len() - 1));
            }
            b'\n' => map.push(vec![]),
            _ => {}
        }
    }

    {
        let mut poi_keys: Vec<_> = poi.keys().skip(1).collect();
        let mut min = 10000;
        loop {
            let mut s: u32 = poi_keys.iter()
                .tuple_windows()
                .map(|(a, b)| find_path_to(&map, &poi[a], &poi[b]).unwrap())
                .sum();
            s += find_path_to(&map, &poi[&0], &poi[poi_keys[0]]).unwrap();
            min = std::cmp::min(min, s);
            if !poi_keys.next_permutation() {
                break;
            }
        }
        println!("Part 1: {}", min);
    }
    {
        let mut poi_keys: Vec<_> = poi.keys().skip(1).collect();
        let mut min = 10000;
        loop {
            let mut s: u32 = poi_keys.iter()
                .tuple_windows()
                .map(|(a, b)| find_path_to(&map, &poi[a], &poi[b]).unwrap())
                .sum();
            s += find_path_to(&map, &poi[&0], &poi[poi_keys[0]]).unwrap();
            s += find_path_to(&map, &poi[&0], &poi[poi_keys.last().unwrap()]).unwrap();
            min = std::cmp::min(min, s);
            if !poi_keys.next_permutation() {
                break;
            }
        }
        println!("Part 2: {}", min);
    }
}
