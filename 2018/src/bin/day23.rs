use regex::Regex;
use std::io::{stdin, BufRead};

fn read_next<'a, T: std::str::FromStr>(
    iter: &mut impl Iterator<Item = regex::Captures<'a>>,
) -> Option<T> {
    iter.next()?.get(0)?.as_str().parse().ok()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Bot {
    pos: (isize, isize, isize),
    r: isize,
}

impl Bot {
    fn in_range_of_other(&self, other: &Self) -> bool {
        self.distance_to(&other.pos) < self.r
    }

    fn mutually_in_range(&self, other: &Self) -> bool {
        let distance = self.distance_to(&other.pos);
        distance < self.r + other.r
    }

    fn distance_to(&self, pos: &(isize, isize, isize)) -> isize {
        (self.pos.0 - pos.0).abs() + (self.pos.1 - pos.1).abs() + (self.pos.2 - pos.2).abs()
    }
}

fn main() {
    let re = Regex::new(r"-?\d+").unwrap();
    let bots: Vec<Bot> = stdin()
        .lock()
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|line| {
            let mut caps = re.captures_iter(&line);
            Some(Bot {
                pos: (
                    read_next(&mut caps)?,
                    read_next(&mut caps)?,
                    read_next(&mut caps)?,
                ),
                r: read_next(&mut caps)?,
            })
        })
        .collect();

    let strongest_bot = bots.iter().max_by_key(|b| b.r).unwrap();
    println!(
        "Part 1: {}",
        bots.iter()
            .filter(|b| strongest_bot.in_range_of_other(b))
            .count()
    );

    let origin = (0, 0, 0);
    let max_r = bots
        .iter()
        .map(|bot| bot.distance_to(&origin))
        .max()
        .unwrap();
    // let max_r = 1 << 4;
    let mut papa_bot = Bot {
        pos: origin,
        r: max_r,
    };

    while papa_bot.r / 2 > 0 {
        let next_r = papa_bot.r / 2;
        papa_bot = [
            (0, 0, 0),
            (-1, 0, 0),
            (0, -1, 0),
            (0, 0, -1),
            (-1, -1, 0),
            (-1, 0, -1),
            (0, -1, -1),
            (-1, -1, -1),
            (1, 0, 0),
            (0, 1, 0),
            (0, 0, 1),
            (1, 1, 0),
            (1, 0, 1),
            (0, 1, 1),
            (1, 1, 1),
        ]
        .iter()
        .map(|(x, y, z)| {
            let sub_papa_bot = Bot {
                pos: (
                    papa_bot.pos.0 + next_r * x,
                    papa_bot.pos.1 + next_r * y,
                    papa_bot.pos.2 + next_r * z,
                ),
                r: next_r,
            };

            let count = bots
                .iter()
                .filter(|bot| bot.mutually_in_range(&sub_papa_bot))
                .count();
            (count, sub_papa_bot.distance_to(&origin), sub_papa_bot)
        })
        .max_by_key(|&(count, distance, _)| (count, -distance))
        .map(|(_, _, sub_papa_bot)| sub_papa_bot)
        .unwrap();
    }

    println!("Part 2: {}", papa_bot.distance_to(&origin));
}
