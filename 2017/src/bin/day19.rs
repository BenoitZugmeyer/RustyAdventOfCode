use std::io::{stdin, Read};
use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
struct Coords {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
enum Direction {
    South,
    North,
    East,
    West,
}

impl std::ops::Add<Direction> for Coords {
    type Output = Coords;

    fn add(self, other: Direction) -> Coords {
        match other {
            Direction::South => Coords {
                x: self.x,
                y: self.y + 1,
            },
            Direction::North => Coords {
                x: self.x,
                y: self.y - 1,
            },
            Direction::East => Coords {
                x: self.x + 1,
                y: self.y,
            },
            Direction::West => Coords {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

#[derive(Debug)]
enum Tile {
    Straight,
    Turn,
    Letter(char),
}

fn main() {
    let mut tiles: HashMap<Coords, Tile> = HashMap::new();
    let mut entry: Option<Coords> = None;
    let mut coords = Coords { x: 0, y: 0 };
    for b in stdin().bytes().flat_map(|a| a.ok()) {
        match b {
            b'|' | b'-' => {
                if coords.y == 0 {
                    entry = Some(coords.clone());
                }
                tiles.insert(coords.clone(), Tile::Straight);
            }
            b'+' => {
                tiles.insert(coords.clone(), Tile::Turn);
            }
            ch if (ch as char).is_alphabetic() => {
                tiles.insert(coords.clone(), Tile::Letter(ch as char));
            }
            b' ' | b'\n' => {}
            _ => panic!("Unexpected char {:?}", b as char),
        }
        match b {
            b'\n' => {
                coords.x = 0;
                coords.y += 1;
            }
            _ => {
                coords.x += 1;
            }
        }
    }

    let mut coords = entry.clone().unwrap();
    let mut direction = Direction::South;
    let mut chars: Vec<char> = Vec::new();
    let mut steps = 0;

    while let Some(tile) = tiles.get(&coords) {
        match *tile {
            Tile::Straight => {}
            Tile::Turn => {
                direction = match direction {
                    Direction::South | Direction::North => {
                        if tiles.contains_key(&(coords + Direction::East)) {
                            Direction::East
                        } else {
                            Direction::West
                        }
                    }
                    Direction::East | Direction::West => {
                        if tiles.contains_key(&(coords + Direction::South)) {
                            Direction::South
                        } else {
                            Direction::North
                        }
                    }
                };
            }
            Tile::Letter(ch) => {
                chars.push(ch);
            }
        }
        coords = coords + direction;
        steps += 1;
    }
    println!("Part 1: {}", chars.iter().collect::<String>());
    println!("Part 2: {}", steps);
}
