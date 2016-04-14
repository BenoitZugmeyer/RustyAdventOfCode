#![feature(iter_arith)]

use std::io;
use std::io::BufRead;
use std::cmp::min;

fn min_configuration(a: Option<(usize, u64)>, b: Option<(usize, u64)>) -> Option<(usize, u64)> {
    match (a, b) {
        (Some(conf_a), Some(conf_b)) => Some(min(conf_a, conf_b)),
        _ => a.or(b),
    }
}

fn pwet(group_weight: u64, package_weights: &[u64], group: &mut Vec<u64>, step: usize) -> Option<(usize, u64)> {
    let sum = group.iter().sum::<u64>();
    let mut result = None;

    for (index, weight) in package_weights.iter().enumerate() {

        if sum + weight > group_weight { continue }

        group.push(*weight);

        result = if sum + weight == group_weight {
            min_configuration(result, Some((group.len(), group.iter().product::<u64>())))
        }
        else {
            min_configuration(result, pwet(group_weight, &package_weights[index + 1..], group, step + 1))
        };

        group.pop();
    }

    result
}

fn find_ideal_configuration(package_weights: &mut Vec<u64>, groups_count: u64) -> Result<(usize, u64), String> {
    let sum: u64 = package_weights.iter().sum();
    if sum % groups_count != 0 { return Err(format!("Total weight is not a multiple of {}", groups_count)) }
    let group_weight = sum / groups_count;
    let mut group = Vec::new();

    // package_weights.sort();
    // package_weights.reverse();

    pwet(group_weight, package_weights, &mut group, 0).ok_or("No result".to_string())
}


#[test]
fn example1() {
    let mut package_weights = (1..6).chain(7..12).collect::<Vec<u64>>();
    assert_eq!(find_ideal_configuration(&mut package_weights, 3), Ok((2, 99)));
}

#[test]
fn example2() {
    let mut package_weights = (1..6).chain(7..12).collect::<Vec<u64>>();
    assert_eq!(find_ideal_configuration(&mut package_weights, 4), Ok((2, 44)));
}

fn main() {
    let stdin = io::stdin();

    let mut package_weights = stdin.lock().lines()
        .filter_map(|l| l.ok())
        .filter_map(|line| line.parse::<u64>().ok())
        .collect::<Vec<u64>>();

    match find_ideal_configuration(&mut package_weights, 3) {
        Ok((_, qe)) => println!("3 packages: quantum entanglement of the first group of packages: {}", qe),
        Err(s) => println!("Error: {}", s),
    }

    match find_ideal_configuration(&mut package_weights, 4) {
        Ok((_, qe)) => println!("4 packages: quantum entanglement of the first group of packages: {}", qe),
        Err(s) => println!("Error: {}", s),
    }
}

