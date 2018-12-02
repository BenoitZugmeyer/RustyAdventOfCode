extern crate regex;
use regex::Regex;
use std::io::stdin;
use std::io::BufRead;

#[derive(Debug)]
struct Disc {
    positions: u8,
    position: u8,
}

impl Disc {
    fn has_slot_up(&self, time: u32) -> bool {
        (time + u32::from(self.position)) % u32::from(self.positions) == 0
    }
}

fn get_start_time(discs: &[Disc]) -> Option<u32> {
    (0..).find(|start_time| {
        discs
            .iter()
            .enumerate()
            .all(|(index, disc)| disc.has_slot_up(start_time + index as u32 + 1))
    })
}

fn main() {
    let re = Regex::new(r"^Disc #\d+ has (\d+) positions; at time=0, it is at position (\d+)\.$")
        .unwrap();

    let stdin = stdin();
    let mut discs: Vec<_> = stdin
        .lock()
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|line| {
            re.captures(&line).and_then(|caps| {
                Some(Disc {
                    positions: caps.at(1).unwrap().parse().unwrap(),
                    position: caps.at(2).unwrap().parse().unwrap(),
                })
            })
        })
        .collect();

    println!("Part 1: {:?}", get_start_time(&discs));
    discs.push(Disc {
        positions: 11,
        position: 0,
    });
    println!("Part 2: {:?}", get_start_time(&discs));
}
