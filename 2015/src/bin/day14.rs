extern crate regex;

use regex::Regex;
use std::cmp::min;
use std::io;
use std::io::BufRead;

fn compute_distance(race_time: u32, kms: u32, run_time: u32, rest_time: u32) -> u32 {
    let sessions_count = race_time / (run_time + rest_time);
    let remaining_time = race_time % (run_time + rest_time);
    (min(run_time, remaining_time) + sessions_count * run_time) * kms
}

fn main() {
    let re = Regex::new(
        r"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds\.",
    )
    .unwrap();
    let race_time = 2503;

    let parameters: Vec<_> = io::stdin()
        .lock()
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|ref line| {
            re.captures(line).map(|ref m| {
                (
                    m.at(2).unwrap().parse::<u32>().unwrap(),
                    m.at(3).unwrap().parse::<u32>().unwrap(),
                    m.at(4).unwrap().parse::<u32>().unwrap(),
                )
            })
        })
        .collect();

    let max = parameters
        .iter()
        .map(|&(kms, run_time, rest_time)| compute_distance(race_time, kms, run_time, rest_time))
        .max()
        .unwrap();

    println!("max: {}", max);

    let mut scores = vec![0; parameters.len()];

    for r in 1..=race_time {
        let distances: Vec<_> = parameters
            .iter()
            .map(|&(kms, run_time, rest_time)| compute_distance(r, kms, run_time, rest_time))
            .collect();

        let max = distances.iter().max().unwrap();
        for (index, d) in distances.iter().enumerate() {
            if d == max {
                scores[index] += 1;
            }
        }
    }

    println!("max score: {}", scores.iter().max().unwrap());
}
