use crate::point::{Direction, Point};
use std::collections::{HashMap, HashSet};
use std::convert::{TryFrom, TryInto};

type Map = Vec<Vec<char>>;
type MapSlice = [Vec<char>];

#[allow(dead_code)]
fn parse_map<S: AsRef<str>, I: Iterator<Item = S>>(iterator: I) -> (Map, Vec<Point>) {
    let mut positions = Vec::new();
    let mut y = 0;
    let map = iterator
        .map(|s| {
            let row: Vec<_> = s.as_ref().chars().collect();
            for (x, b) in row.iter().enumerate() {
                if *b == '@' {
                    positions.push(Point::new(x.try_into().unwrap(), y));
                }
            }
            y += 1;
            row
        })
        .collect();

    (map, positions)
}

fn should_continue<K: std::cmp::Eq + std::hash::Hash>(
    key: K,
    steps: u32,
    discovered: &mut HashMap<K, u32>,
) -> bool {
    if let Some(prev_steps) = discovered.get(&key) {
        if *prev_steps <= steps {
            return false;
        }
    }
    discovered.insert(key, steps);
    true
}

fn discover_keys_and_doors(
    map: &MapSlice,
    position: Point,
    discovered: &mut HashSet<Point>,
    doors_on_path: &[char],
    keys: &mut Vec<(char, Point, Vec<char>)>,
) {
    discovered.insert(position);

    for direction in Direction::iter() {
        let new_position = position.walk(direction);
        if discovered.contains(&new_position) {
            continue;
        }
        match map[usize::try_from(new_position.y).unwrap()]
            [usize::try_from(new_position.x).unwrap()]
        {
            '#' => {} // wall
            '.' | '@' => {
                discover_keys_and_doors(map, new_position, discovered, doors_on_path, keys);
            }
            b @ 'A'..='Z' => {
                let mut doors_on_path = doors_on_path.to_vec();
                doors_on_path.push(b.to_ascii_lowercase());
                discover_keys_and_doors(map, new_position, discovered, &doors_on_path, keys);
            }
            b @ 'a'..='z' => {
                keys.push((b, new_position, doors_on_path.to_vec()));
                discover_keys_and_doors(map, new_position, discovered, doors_on_path, keys);
            }
            b => panic!("Unknown tile {}", b as char),
        }
    }
}

fn find_shortest_path_rec(
    keys: &[(char, Vec<char>)],
    available_keys: &HashSet<char>,
    steps: u32,
    costs: &HashMap<char, HashMap<char, u32>>,
    position: char,
    discovered: &mut HashMap<(char, Vec<char>), u32>,
) -> Option<u32> {
    let mut a: Vec<_> = available_keys.iter().cloned().collect();
    a.sort();
    if !should_continue((position, a), steps, discovered) {
        return None;
    }

    if available_keys.len() == keys.len() {
        return Some(steps);
    }

    let mut min_steps: Option<u32> = None;
    for (key, requirements) in keys {
        if !available_keys.contains(key) && requirements.iter().all(|r| available_keys.contains(r))
        {
            let mut available_keys = available_keys.clone();
            available_keys.insert(*key);
            if let Some(steps) = find_shortest_path_rec(
                keys,
                &available_keys,
                steps + costs[&position][&key],
                costs,
                *key,
                discovered,
            ) {
                min_steps = Some(match min_steps {
                    Some(min_steps) => min_steps.min(steps),
                    None => steps,
                });
            }
        }
    }

    min_steps
}

fn find_shortest_path_multi_position_rec(
    keys: &[(usize, char, Vec<char>)],
    available_keys: &HashSet<char>,
    steps: u32,
    costs: &HashMap<char, HashMap<char, u32>>,
    positions: &[char],
    discovered: &mut HashMap<(Vec<char>, Vec<char>), u32>,
) -> Option<u32> {
    let discovered_key = {
        let mut a: Vec<_> = available_keys.iter().cloned().collect();
        a.sort();
        (positions.to_vec(), a)
    };
    if !should_continue(discovered_key, steps, discovered) {
        return None;
    }

    if available_keys.len() == keys.len() {
        return Some(steps);
    }

    let mut min_steps: Option<u32> = None;
    for (position_index, key, requirements) in keys {
        if !available_keys.contains(key) && requirements.iter().all(|r| available_keys.contains(r))
        {
            let mut available_keys = available_keys.clone();
            available_keys.insert(*key);
            let mut new_positions = positions.to_vec();
            new_positions[*position_index] = *key;
            if let Some(steps) = find_shortest_path_multi_position_rec(
                keys,
                &available_keys,
                steps + costs[&positions[*position_index]][&key],
                costs,
                &new_positions,
                discovered,
            ) {
                min_steps = Some(match min_steps {
                    Some(min_steps) => min_steps.min(steps),
                    None => steps,
                });
            }
        }
    }

    min_steps
}

fn discover(map: &MapSlice, position: Point, discovered: &mut HashMap<Point, u32>, steps: u32) {
    if !should_continue(position, steps, discovered) {
        return;
    }
    for direction in Direction::iter() {
        let new_position = position.walk(direction);
        match map[usize::try_from(new_position.y).unwrap()]
            [usize::try_from(new_position.x).unwrap()]
        {
            '#' => {} // wall
            '.' | '@' | 'A'..='Z' | 'a'..='z' => {
                discover(map, new_position, discovered, steps + 1);
            }
            b => panic!("Unknown tile {}", b as char),
        }
    }
}

fn compute_costs(
    map: &MapSlice,
    keys: &[(char, Point, Vec<char>)],
    position: Point,
) -> HashMap<char, u32> {
    let mut discovered = HashMap::new();
    discover(map, position, &mut discovered, 0);
    keys.iter()
        .map(|(key, position, _)| (*key, discovered[position]))
        .collect()
}

#[allow(dead_code)]
fn find_shortest_path(map: &MapSlice, position: Point) -> Option<u32> {
    let mut keys = Vec::new();
    discover_keys_and_doors(map, position, &mut HashSet::new(), &[], &mut keys);
    let mut costs: HashMap<char, HashMap<char, u32>> = HashMap::new();

    costs.insert('@', compute_costs(map, &keys, position));
    for (key, position, _) in &keys {
        costs.insert(*key, compute_costs(map, &keys, *position));
    }
    let keys: Vec<_> = keys
        .into_iter()
        .map(|(key, _, requirements)| (key, requirements))
        .collect();

    find_shortest_path_rec(&keys, &HashSet::new(), 0, &costs, '@', &mut HashMap::new())
}

fn usize_to_char(n: usize) -> char {
    (b'0' + u8::try_from(n).unwrap()) as char
}

#[allow(dead_code)]
fn find_shortest_path_multi_position(map: &MapSlice, positions: &[Point]) -> Option<u32> {
    let mut costs: HashMap<char, HashMap<char, u32>> = HashMap::new();

    let mut all_keys = Vec::new();
    for (i, position) in positions.iter().enumerate() {
        let mut keys = Vec::new();
        discover_keys_and_doors(&map, *position, &mut HashSet::new(), &[], &mut keys);
        costs.insert(usize_to_char(i), compute_costs(map, &keys, *position));
        for (key, position, _) in &keys {
            costs.insert(*key, compute_costs(map, &keys, *position));
        }
        all_keys.extend(
            keys.into_iter()
                .map(|(key, _, requirements)| (i, key, requirements)),
        );
    }

    find_shortest_path_multi_position_rec(
        &all_keys,
        &HashSet::new(),
        0,
        &costs,
        &positions
            .iter()
            .enumerate()
            .map(|(index, _)| usize_to_char(index))
            .collect::<Vec<_>>(),
        &mut HashMap::new(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    // 5064
    #[test]
    fn test_1_1() {
        let (map, positions) = parse_map(
            "#########\n\
             #b.A.@.a#\n\
             #########"
                .split('\n'),
        );

        assert_eq!(find_shortest_path(&map, positions[0]), Some(8));
    }

    #[test]
    fn test_1_2() {
        let (map, positions) = parse_map(
            "########################\n\
             #f.D.E.e.C.b.A.@.a.B.c.#\n\
             ######################.#\n\
             #d.....................#\n\
             ########################"
                .split('\n'),
        );

        assert_eq!(find_shortest_path(&map, positions[0]), Some(86));
    }

    #[test]
    fn test_1_3() {
        let (map, positions) = parse_map(
            "########################\n\
             #...............b.C.D.f#\n\
             #.######################\n\
             #.....@.a.B.c.d.A.e.F.g#\n\
             ########################"
                .split('\n'),
        );

        assert_eq!(find_shortest_path(&map, positions[0]), Some(132));
    }

    #[test]
    fn test_1_4() {
        let (map, positions) = parse_map(
            "#################\n\
             #i.G..c...e..H.p#\n\
             ########.########\n\
             #j.A..b...f..D.o#\n\
             ########@########\n\
             #k.E..a...g..B.n#\n\
             ########.########\n\
             #l.F..d...h..C.m#\n\
             #################"
                .split('\n'),
        );

        assert_eq!(find_shortest_path(&map, positions[0]), Some(136));
    }

    #[test]
    fn test_1_5() {
        let (map, positions) = parse_map(
            "########################\n\
             #@..............ac.GI.b#\n\
             ###d#e#f################\n\
             ###A#B#C################\n\
             ###g#h#i################\n\
             ########################"
                .split('\n'),
        );

        assert_eq!(find_shortest_path(&map, positions[0]), Some(81));
    }

    #[test]
    fn part_1() {
        let (map, positions) = parse_map(util::input(18));

        assert_eq!(find_shortest_path(&map, positions[0]), util::answer(18, 1));
    }

    #[test]
    fn test_2_1() {
        let (map, positions) = parse_map(
            "#######\n\
             #a.#Cd#\n\
             ##@#@##\n\
             #######\n\
             ##@#@##\n\
             #cB#Ab#\n\
             #######"
                .split('\n'),
        );
        assert_eq!(find_shortest_path_multi_position(&map, &positions), Some(8));
    }

    #[test]
    fn test_2_2() {
        let (map, positions) = parse_map(
            "#############\n\
             #g#f.D#..h#l#\n\
             #F###e#E###.#\n\
             #dCba@#@BcIJ#\n\
             #############\n\
             #nK.L@#@G...#\n\
             #M###N#H###.#\n\
             #o#m..#i#jk.#\n\
             #############"
                .split('\n'),
        );
        assert_eq!(
            find_shortest_path_multi_position(&map, &positions),
            Some(72)
        );
    }

    #[test]
    fn part_2() {
        let (mut map, positions) = parse_map(util::input(18));
        let center = positions[0];
        map[usize::try_from(center.y).unwrap()][usize::try_from(center.x).unwrap()] = '#';
        for direction in Direction::iter() {
            let p = center.walk(direction);
            map[usize::try_from(p.y).unwrap()][usize::try_from(p.x).unwrap()] = '#';
        }
        let positions = [
            center.walk(Direction::East).walk(Direction::South),
            center.walk(Direction::East).walk(Direction::North),
            center.walk(Direction::West).walk(Direction::South),
            center.walk(Direction::West).walk(Direction::North),
        ];
        assert_eq!(
            find_shortest_path_multi_position(&map, &positions),
            util::answer(18, 2)
        );
    }
}
