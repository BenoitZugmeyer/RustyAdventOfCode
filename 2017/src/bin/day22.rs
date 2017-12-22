use std::io::{stdin, BufRead};
use std::collections::HashMap;

type Coord = (i32, i32);

#[derive(Debug)]
enum Direction {
    North,
    East,
    West,
    South,
}

impl Direction {
    #[inline]
    fn turn_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }
    #[inline]
    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
    #[inline]
    fn move_(&self, coord: &Coord) -> Coord {
        match self {
            Direction::North => (coord.0, coord.1 - 1),
            Direction::East => (coord.0 + 1, coord.1),
            Direction::South => (coord.0, coord.1 + 1),
            Direction::West => (coord.0 - 1, coord.1),
        }
    }
}

#[derive(Debug, Clone)]
enum NodeState {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl NodeState {
    #[inline]
    fn direct_mutate(&mut self) {
        *self = match self {
            NodeState::Clean => NodeState::Infected,
            NodeState::Infected => NodeState::Clean,
            _ => panic!("Can't direct mutate a intermediary state"),
        }
    }
    #[inline]
    fn mutate(&mut self) {
        *self = match self {
            NodeState::Clean => NodeState::Weakened,
            NodeState::Weakened => NodeState::Infected,
            NodeState::Infected => NodeState::Flagged,
            NodeState::Flagged => NodeState::Clean,
        }
    }
}

fn run(
    mut current_node: Coord,
    map: HashMap<Coord, NodeState>,
    speed: u32,
    direct_mutate: bool,
) -> u32 {
    let mut map = map.clone();
    let mut direction = Direction::North;
    let mut burst_infection = 0;

    for _ in 0..speed {
        let current_node_infection = map.entry(current_node).or_insert(NodeState::Clean);
        direction = match *current_node_infection {
            NodeState::Clean => direction.turn_left(),
            NodeState::Weakened => direction,
            NodeState::Infected => direction.turn_right(),
            NodeState::Flagged => direction.turn_left().turn_left(),
        };
        if direct_mutate {
            current_node_infection.direct_mutate();
        } else {
            current_node_infection.mutate();
        }
        match *current_node_infection {
            NodeState::Infected => burst_infection += 1,
            _ => {}
        }
        current_node = direction.move_(&current_node);
    }

    burst_infection
}

fn main() {
    let stdin = stdin();

    let parsed_map: Vec<Vec<NodeState>> = stdin
        .lock()
        .lines()
        .filter_map(|l| l.ok())
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|ch| {
                    if ch == '#' {
                        NodeState::Infected
                    } else {
                        NodeState::Clean
                    }
                })
                .collect()
        })
        .collect();

    let map: HashMap<Coord, NodeState> = parsed_map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, infected)| ((x as i32, y as i32), infected.clone()))
                .collect::<Vec<(Coord, NodeState)>>()
        })
        .collect();

    let current_node: Coord = (parsed_map[0].len() as i32 / 2, parsed_map.len() as i32 / 2);

    println!(
        "Part 1: {}",
        run(current_node.clone(), map.clone(), 10_000, true)
    );

    println!(
        "Part 2: {}",
        run(current_node.clone(), map.clone(), 10_000_000, false)
    );
}
