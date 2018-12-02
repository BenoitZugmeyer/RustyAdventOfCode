use std::io;
use std::io::Read;
use std::collections::btree_set::BTreeSet;

struct Coordinates {
    x: i32,
    y: i32,
}

impl Coordinates {
    fn new() -> Self {
        Coordinates { x: 0, y: 0 }
    }

    fn apply(&mut self, direction: u8) {
        match direction {
            b'<' => self.x -= 1,
            b'>' => self.x += 1,
            b'v' => self.y -= 1,
            b'^' => self.y += 1,
            _ => {},
        };
    }
}

impl Default for Coordinates {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    let mut santa_first_year = Coordinates::new();
    let mut santa = Coordinates::new();
    let mut robot = Coordinates::new();

    let mut houses_first_year: BTreeSet<(i32, i32)> = BTreeSet::new();
    let mut houses_next_year: BTreeSet<(i32, i32)> = BTreeSet::new();

    let directions = io::stdin().bytes().filter_map(|ch| ch.ok());

    for (index, ch) in directions.enumerate() {
        santa_first_year.apply(ch);
        houses_first_year.insert((santa_first_year.x, santa_first_year.y));

        let coords_next_year = if index % 2 == 0 { &mut santa } else { &mut robot };
        coords_next_year.apply(ch);
        houses_next_year.insert((coords_next_year.x, coords_next_year.y));
    }

    println!("Total first year: {}", houses_first_year.len());
    println!("Total next year: {}", houses_next_year.len());

}
