use std::cmp;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::io::stdin;
use std::io::Read;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone)]
struct Coords {
    x: u32,
    y: u32,
}

impl Coords {
    fn new(x: u32, y: u32) -> Self {
        Coords { x, y }
    }

    fn is_wall(&self, favorite_number: u32) -> bool {
        let n = self.x * self.x
            + 3 * self.x
            + 2 * self.x * self.y
            + self.y
            + self.y * self.y
            + favorite_number;
        n.count_ones() % 2 == 1
    }

    fn next(&self, favorite_number: u32) -> Vec<Coords> {
        [(0, 1), (2, 1), (1, 0), (1, 2)]
            .iter()
            .filter(|&&(dx, dy)| (dx > 0 || self.x > 0) && (dy > 0 || self.y > 0))
            .map(|&(dx, dy)| Coords::new(self.x + dx - 1, self.y + dy - 1))
            .filter(|c| !c.is_wall(favorite_number))
            .collect()
    }
}

fn print(max: &Coords, favorite_number: u32, path: &BTreeSet<Coords>) {
    print!("   ");
    for x in 0..(max.x + 5) {
        print!("{:2}", x);
    }
    println!();

    for y in 0..(max.y + 5) {
        print!("{:2} ", y);
        for x in 0..(max.x + 5) {
            let c = Coords::new(x, y);
            if c.is_wall(favorite_number) {
                print!(" #");
            } else if path.contains(&c) {
                print!(" O");
            } else {
                print!(" .");
            }
        }
        println!();
    }
}

fn find_path(
    position: &Coords,
    destination: &Coords,
    favorite_number: u32,
    mut previous_positions: &mut BTreeMap<Coords, u32>,
    steps: u32,
) -> Option<u32> {
    if let Some(s) = previous_positions.get(position) {
        if *s <= steps {
            return None;
        }
    }

    previous_positions.insert(position.clone(), steps);

    if position == destination {
        Some(steps)
    } else {
        let mut shortest_path: Option<u32> = None;
        for coord in &position.next(favorite_number) {
            if let Some(path) = find_path(
                coord,
                destination,
                favorite_number,
                &mut previous_positions,
                steps + 1,
            ) {
                shortest_path = Some(cmp::min(path, shortest_path.unwrap_or(1000)));
            }
        }
        shortest_path
    }
}

fn find_reachable_path(
    position: &Coords,
    favorite_number: u32,
    mut previous_positions: &mut BTreeMap<Coords, u32>,
    steps: u32,
) {
    if let Some(s) = previous_positions.get(position) {
        if *s >= steps {
            return;
        }
    }

    previous_positions.insert(position.clone(), steps);
    if steps > 0 {
        for coord in &position.next(favorite_number) {
            find_reachable_path(coord, favorite_number, &mut previous_positions, steps - 1);
        }
    }
}

fn main() {
    let input = stdin()
        .bytes()
        .filter_map(|b| {
            b.ok().and_then(|b| match b {
                b'0'...b'9' => Some(u32::from(b - b'0')),
                _ => None,
            })
        })
        .fold(0u32, |acc, n| acc * 10 + n);

    if let Some(path) = find_path(
        &Coords::new(1, 1),
        &Coords::new(31, 39),
        input,
        &mut BTreeMap::new(),
        0,
    ) {
        println!("Part 1: {}", path);
    }

    let mut positions = BTreeMap::new();
    find_reachable_path(&Coords::new(1, 1), input, &mut positions, 50);
    print(
        &Coords::new(20, 20),
        input,
        &positions.keys().cloned().collect(),
    );
    println!("Part 2: {}", positions.len());
}
