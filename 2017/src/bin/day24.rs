use std::io::{stdin, BufRead};
use std::collections::HashSet;

type Component = (u8, u8);

fn find_strongest_bridge(
    last_port: u8,
    components: &HashSet<Component>,
    used_components: &mut HashSet<Component>,
) -> u32 {
    let mut max = 0;
    for component in components {
        if component.0 != last_port && component.1 != last_port {
            continue;
        }
        if used_components.contains(component) {
            continue;
        }
        used_components.insert(*component);
        let next_port = if component.0 == last_port {
            component.1
        } else {
            component.0
        };
        let score = find_strongest_bridge(next_port, components, used_components)
            + component.1 as u32 + component.0 as u32;
        max = std::cmp::max(score, max);
        used_components.remove(component);
    }
    max
}

fn find_strongest_longest_bridge(
    last_port: u8,
    components: &HashSet<Component>,
    used_components: &mut HashSet<Component>,
) -> (u32, u32) {
    let mut max = (0, 0);
    for component in components {
        if component.0 != last_port && component.1 != last_port {
            continue;
        }
        if used_components.contains(component) {
            continue;
        }
        used_components.insert(*component);
        let next_port = if component.0 == last_port {
            component.1
        } else {
            component.0
        };
        let (mut length, mut score) =
            find_strongest_longest_bridge(next_port, components, used_components);
        score += component.1 as u32 + component.0 as u32;
        length += 1;
        max = std::cmp::max((length, score), max);
        used_components.remove(component);
    }
    max
}

fn main() {
    let stdin = stdin();
    let components: HashSet<(u8, u8)> = stdin
        .lock()
        .lines()
        .filter_map(|l| l.ok())
        .flat_map(|line| {
            line.find('/').map(|pos| {
                (
                    line[0..pos].parse().unwrap(),
                    line[pos + 1..].parse().unwrap(),
                )
            })
        })
        .collect();

    {
        let mut used_components = HashSet::new();
        println!(
            "Part 1: {}",
            find_strongest_bridge(0, &components, &mut used_components)
        );
    }

    {
        let mut used_components = HashSet::new();
        println!(
            "Part 2: {}",
            find_strongest_longest_bridge(0, &components, &mut used_components).1
        );
    }
}
