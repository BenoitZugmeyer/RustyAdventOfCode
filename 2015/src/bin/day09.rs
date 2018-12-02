extern crate permutohedron;
extern crate regex;

use permutohedron::Heap;
use regex::Regex;
use std::cmp::{max, min};
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::io;
use std::io::BufRead;

fn new_path(from: &str, to: &str) -> BTreeSet<String> {
    let mut path = BTreeSet::new();
    path.insert(String::from(from));
    path.insert(String::from(to));
    path
}

fn main() {
    let mut all_cities = BTreeSet::new();
    let mut distances = BTreeMap::new();

    let re = Regex::new(r"(\w+) to (\w+) = (\d+)").unwrap();

    for line in io::stdin().lock().lines().filter_map(|l| l.ok()) {
        if let Some(ref m) = re.captures(&line) {
            all_cities.insert(String::from(m.at(1).unwrap()));
            all_cities.insert(String::from(m.at(2).unwrap()));
            let path = new_path(m.at(1).unwrap(), m.at(2).unwrap());
            let distance = m.at(3).unwrap().parse::<u32>().unwrap();
            distances.insert(path, distance);
        }
    }

    let mut all_cities_vec = all_cities.iter().collect::<Vec<_>>();

    let (max, min) = Heap::new(&mut all_cities_vec)
        .map(|cities| {
            let mut cities_iter = cities.iter();
            let starting_city = cities_iter.next().unwrap();
            cities_iter
                .scan(starting_city, |previous_city, city| {
                    let distance = distances.get(&new_path(previous_city, city));
                    *previous_city = city;
                    distance
                })
                .sum::<u32>()
        })
        .fold((u32::min_value(), u32::max_value()), |(max_d, min_d), d| {
            (max(max_d, d), min(min_d, d))
        });

    println!("Shortest route: {}", min);
    println!("Longest route: {}", max);
}
