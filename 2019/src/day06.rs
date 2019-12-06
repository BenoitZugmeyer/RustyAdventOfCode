use std::collections::HashMap;

type Map = HashMap<String, Vec<String>>;

fn iter_orbits<'a>(center: &str, map: &'a Map) -> impl Iterator<Item = &'a String> {
    map.get(center).into_iter().flat_map(|orbits| orbits.iter())
}

fn count_suborbits(center: &str, map: &Map) -> u32 {
    iter_orbits(center, map)
        .map(|orbit| count_suborbits(orbit, map) + 1)
        .sum()
}

#[allow(dead_code)]
fn count_orbits(center: &str, map: &Map) -> u32 {
    count_suborbits(center, map)
        + iter_orbits(center, map)
            .map(|orbit| count_orbits(orbit, map))
            .sum::<u32>()
}

fn count_orbit_transfers_rec(center: &str, map: &Map) -> (Option<u32>, Option<u32>) {
    if center == "SAN" {
        (None, Some(0))
    } else if center == "YOU" {
        (Some(0), None)
    } else {
        let res = iter_orbits(center, map)
            .map(|orbit| count_orbit_transfers_rec(orbit, map))
            .fold((None, None), |prev, current| {
                (prev.0.or(current.0), prev.1.or(current.1))
            });

        match res {
            (Some(d), None) => (Some(d + 1), None),
            (None, Some(d)) => (None, Some(d + 1)),
            res => res,
        }
    }
}

#[allow(dead_code)]
fn count_orbit_transfers(map: &Map) -> u32 {
    let (a, b) = count_orbit_transfers_rec("COM", map);
    a.unwrap() + b.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn parse_map<T: Iterator<Item = String>>(lines: T) -> Map {
        let mut map = HashMap::new();
        for line in lines {
            let (center, orbit) = line.split_at(line.find(')').expect("Failed to parse line"));
            map.entry(center.to_string())
                .or_insert_with(Vec::new)
                .push(orbit[1..].to_string());
        }
        map
    }

    #[test]
    fn iter_digits_test() {
        assert_eq!(
            parse_map(
                "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L"
                    .split('\n')
                    .map(String::from)
            ),
            [
                ("K".to_string(), vec!("L".to_string())),
                ("J".to_string(), vec!("K".to_string())),
                ("C".to_string(), vec!("D".to_string())),
                ("E".to_string(), vec!("F".to_string(), "J".to_string())),
                ("D".to_string(), vec!("E".to_string(), "I".to_string())),
                ("B".to_string(), vec!("C".to_string(), "G".to_string())),
                ("G".to_string(), vec!("H".to_string())),
                ("COM".to_string(), vec!("B".to_string())),
            ]
            .iter()
            .cloned()
            .collect()
        );
    }

    #[test]
    fn count_orbits_test() {
        let map = parse_map("COM)B\nB)C".split('\n').map(String::from));
        assert_eq!(count_orbits("COM", &map), 3);
        let map = parse_map("COM)B\nB)C\nC)D".split('\n').map(String::from));
        assert_eq!(count_orbits("COM", &map), 6);
        let map = parse_map(
            "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L"
                .split('\n')
                .map(String::from),
        );
        assert_eq!(count_orbits("COM", &map), 42);
    }

    #[test]
    fn part_1() {
        let map = parse_map(util::input(6));
        assert_eq!(Some(count_orbits("COM", &map)), util::answer(6, 1));
    }

    #[test]
    fn count_orbit_transfers_test() {
        let map = parse_map(
            "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN"
                .split('\n')
                .map(String::from),
        );
        assert_eq!(count_orbit_transfers(&map), 4);
    }

    #[test]
    fn part_2() {
        let map = parse_map(util::input(6));
        assert_eq!(Some(count_orbit_transfers(&map)), util::answer(6, 2));
    }
}
