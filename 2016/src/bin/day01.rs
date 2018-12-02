extern crate regex;
use regex::Regex;
use std::collections::BTreeSet;
use std::io::stdin;
use std::io::Read;

#[derive(Ord, Eq, PartialOrd, PartialEq, Copy, Clone, Debug)]
struct Coordinates {
    x: i16,
    y: i16,
}

impl Coordinates {
    fn new(x: i16, y: i16) -> Self {
        Coordinates { x, y }
    }

    fn distance(self) -> i16 {
        self.x.abs() + self.y.abs()
    }

    fn from(Coordinates { x, y }: Self, direction: u16, distance: i16) -> Self {
        match direction {
            0 => Coordinates::new(x, y + distance),
            90 => Coordinates::new(x + distance, y),
            180 => Coordinates::new(x, y - distance),
            270 => Coordinates::new(x - distance, y),
            _ => panic!("Invalid direction"),
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    let re = Regex::new(r"(?P<turn>[RL])(?P<distance>\d+)").unwrap();

    let mut previous_locations = BTreeSet::new();

    let (location, _, cross_location) = re.captures_iter(&input).fold(
        (Coordinates::new(0, 0), 90u16, None),
        |(location, direction, cross_location), caps| {
            let turn = caps.name("turn").unwrap().chars().nth(0).unwrap();
            let distance = caps
                .name("distance")
                .unwrap()
                .parse()
                .expect("Failed to parse distance");

            let new_direction = match turn {
                'L' => direction + 90,
                'R' => direction + 270,
                _ => 0,
            } % 360;

            let new_location = Coordinates::from(location, new_direction, distance);

            let new_cross_location = cross_location.or_else(|| {
                (0..distance)
                    .filter_map(|d| {
                        let tmp_location = Coordinates::from(location, new_direction, d);
                        if previous_locations.insert(tmp_location) {
                            None
                        } else {
                            Some(tmp_location)
                        }
                    })
                    .nth(0)
            });

            (new_location, new_direction, new_cross_location)
        },
    );

    println!("Part 1: {}", location.distance());
    println!("Part 2: {}", cross_location.unwrap().distance());
}
