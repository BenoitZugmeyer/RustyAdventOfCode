use std::io::{stdin, BufRead};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Cell {
    Empty,
    StraightVertical,
    StraightHorizontal,
    TurnNorthLeft,
    TurnNorthRight,
    Intersection,
}

#[derive(Debug, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }
    fn right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coordinates {
    x: usize,
    y: usize,
}

impl Coordinates {
    fn next(&self, direction: &Direction) -> Coordinates {
        match direction {
            Direction::North => Coordinates {
                x: self.x,
                y: self.y - 1,
            },
            Direction::East => Coordinates {
                x: self.x + 1,
                y: self.y,
            },
            Direction::West => Coordinates {
                x: self.x - 1,
                y: self.y,
            },
            Direction::South => Coordinates {
                x: self.x,
                y: self.y + 1,
            },
        }
    }

    fn value<T: Clone>(&self, grid: &[Vec<T>]) -> Option<T> {
        grid.get(self.y)?.get(self.x).cloned()
    }
}

#[derive(Debug, Clone)]
struct Cart {
    coordinates: Coordinates,
    direction: Direction,
    passed_intersections: usize,
}

impl Cart {
    fn new(coordinates: (usize, usize), direction: Direction) -> Self {
        Self {
            coordinates: Coordinates {
                x: coordinates.0,
                y: coordinates.1,
            },
            direction,
            passed_intersections: 0,
        }
    }

    fn move_(&mut self, map: &[Vec<Cell>]) {
        let next_direction = match self.coordinates.value(map).unwrap() {
            Cell::StraightVertical | Cell::StraightHorizontal => self.direction.clone(),
            Cell::Intersection => {
                let result = match self.passed_intersections {
                    0 => self.direction.left(),
                    1 => self.direction.clone(),
                    2 => self.direction.right(),
                    _ => unreachable!(),
                };
                self.passed_intersections = (self.passed_intersections + 1) % 3;
                result
            }
            Cell::TurnNorthRight => {
                // '/'
                match self.direction {
                    Direction::North => Direction::East,
                    Direction::West => Direction::South,
                    Direction::East => Direction::North,
                    Direction::South => Direction::West,
                }
            }
            Cell::TurnNorthLeft => {
                // '\'
                match self.direction {
                    Direction::North => Direction::West,
                    Direction::West => Direction::North,
                    Direction::East => Direction::South,
                    Direction::South => Direction::East,
                }
            }
            Cell::Empty => unreachable!(),
        };
        self.coordinates = self.coordinates.next(&next_direction);
        self.direction = next_direction;
    }
}

fn step(map: &[Vec<Cell>], mut carts: Vec<Cart>) -> (Vec<Cart>, Vec<Cart>) {
    carts.sort_by_key(|cart| (cart.coordinates.y, cart.coordinates.x));

    let mut crashed_carts = Vec::new();
    let mut index = 0;
    while index < carts.len() {
        carts[index].move_(&map);

        if let Some(other_index) = (0..carts.len()).find(|&other_index| {
            index != other_index && carts[index].coordinates == carts[other_index].coordinates
        }) {
            if other_index < index {
                crashed_carts.push(carts.remove(index));
                crashed_carts.push(carts.remove(other_index));
                index -= 1;
            } else {
                crashed_carts.push(carts.remove(other_index));
                crashed_carts.push(carts.remove(index));
            }
        } else {
            index += 1;
        }
    }

    (carts, crashed_carts)
}

fn main() {
    let mut carts: Vec<Cart> = Vec::new();
    let map: Vec<Vec<Cell>> = stdin()
        .lock()
        .lines()
        .filter_map(|l| l.ok())
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, ch)| match ch {
                    '|' => Cell::StraightVertical,
                    '-' => Cell::StraightHorizontal,
                    '+' => Cell::Intersection,
                    ' ' => Cell::Empty,
                    '/' => Cell::TurnNorthRight,
                    '\\' => Cell::TurnNorthLeft,
                    'v' => {
                        carts.push(Cart::new((x, y), Direction::South));
                        Cell::StraightVertical
                    }
                    '>' => {
                        carts.push(Cart::new((x, y), Direction::East));
                        Cell::StraightHorizontal
                    }
                    '<' => {
                        carts.push(Cart::new((x, y), Direction::West));
                        Cell::StraightHorizontal
                    }
                    '^' => {
                        carts.push(Cart::new((x, y), Direction::North));
                        Cell::StraightVertical
                    }
                    _ => panic!("Invalid character {}", ch),
                })
                .collect()
        })
        .collect();

    {
        let mut carts = carts.clone();
        loop {
            let (next_carts, crashed) = step(&map, carts);
            if !crashed.is_empty() {
                println!(
                    "Part 1: {},{}",
                    crashed[0].coordinates.x, crashed[0].coordinates.y
                );
                break;
            }
            carts = next_carts;
        }
    }

    {
        let mut carts = carts.clone();
        while carts.len() > 1 {
            carts = step(&map, carts).0;
        }
        println!(
            "Part 2: {},{}",
            carts[0].coordinates.x, carts[0].coordinates.y
        );
    }
}
