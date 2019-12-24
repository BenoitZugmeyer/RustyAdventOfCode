#[cfg(test)]
mod tests {
    use crate::util;
    use itertools::{iproduct, Itertools};
    use std::collections::HashSet;

    const GRID_SIZE: u32 = 5;
    type Map = u32;

    fn parse_map<S: AsRef<str>, I: Iterator<Item = S>>(iterator: I) -> Map {
        let mut res = 0;
        let mut index = 0;
        for line in iterator {
            for ch in line.as_ref().chars() {
                res |= if ch == '#' { 1 << index } else { 0 };
                index += 1;
            }
        }
        res
    }

    fn has_insect(map: Map, x: u32, y: u32) -> bool {
        map >> (x + y * GRID_SIZE) & 0b1 == 1
    }

    fn step(map: Map) -> Map {
        iproduct!(0..GRID_SIZE, 0..GRID_SIZE).fold(0, |new_map, (y, x)| {
            let x = GRID_SIZE - x - 1;
            let y = GRID_SIZE - y - 1;
            let mut count = 0;
            if y > 0 && has_insect(map, x, y - 1) {
                count += 1;
            }
            if x > 0 && has_insect(map, x - 1, y) {
                count += 1;
            }
            if y < GRID_SIZE - 1 && has_insect(map, x, y + 1) {
                count += 1;
            }
            if x < GRID_SIZE - 1 && has_insect(map, x + 1, y) {
                count += 1;
            }
            let has = has_insect(map, x, y);
            new_map << 1
                | if !has && (count == 1 || count == 2) || has && count == 1 {
                    1
                } else {
                    0
                }
        })
    }

    #[test]
    fn parse_map_test() {
        let map = parse_map(
            "#...#\n\
             .....\n\
             .....\n\
             .....\n\
             ....."
                .split('\n'),
        );

        assert_eq!(map, 0b10001);
    }

    #[test]
    fn test_1() {
        let map = parse_map(
            "....#\n\
             #..#.\n\
             #..##\n\
             ..#..\n\
             #...."
                .split('\n'),
        );

        assert_eq!(
            step(map),
            parse_map(
                "#..#.\n\
                 ####.\n\
                 ###.#\n\
                 ##.##\n\
                 .##.."
                    .split('\n')
            )
        );
    }

    #[test]
    fn part_1() {
        let mut map = parse_map(util::input(24));

        let mut layouts = HashSet::new();

        while !layouts.contains(&map) {
            layouts.insert(map);
            map = step(map);
        }

        assert_eq!(Some(map), util::answer(24, 1));
    }

    fn step_rec(levels: &[Map]) -> Vec<Map> {
        let mut result = vec![0, 0];
        for (superior, level, inferior) in levels.iter().cloned().tuple_windows() {
            let new_level = iproduct!(0..GRID_SIZE, 0..GRID_SIZE).fold(0, |new_map, (y, x)| {
                if x == 2 && y == 2 {
                    return new_map << 1;
                }

                let x = GRID_SIZE - x - 1;
                let y = GRID_SIZE - y - 1;
                let mut count = 0;
                if y == 0 && has_insect(superior, 2, 1) {
                    count += 1;
                }
                if x == 0 && has_insect(superior, 1, 2) {
                    count += 1;
                }
                if y == GRID_SIZE - 1 && has_insect(superior, 2, 3) {
                    count += 1;
                }
                if x == GRID_SIZE - 1 && has_insect(superior, 3, 2) {
                    count += 1;
                }
                if x == 1 && y == 2 {
                    count += (0..GRID_SIZE)
                        .filter(|i| has_insect(inferior, 0, *i))
                        .count()
                } else if x < GRID_SIZE - 1 && has_insect(level, x + 1, y) {
                    count += 1;
                }
                if x == 2 && y == 1 {
                    count += (0..GRID_SIZE)
                        .filter(|i| has_insect(inferior, *i, 0))
                        .count()
                } else if y < GRID_SIZE - 1 && has_insect(level, x, y + 1) {
                    count += 1;
                }
                if x == 3 && y == 2 {
                    count += (0..GRID_SIZE)
                        .filter(|i| has_insect(inferior, GRID_SIZE - 1, *i))
                        .count()
                } else if x > 0 && has_insect(level, x - 1, y) {
                    count += 1;
                }
                if x == 2 && y == 3 {
                    count += (0..GRID_SIZE)
                        .filter(|i| has_insect(inferior, *i, GRID_SIZE - 1))
                        .count()
                } else if y > 0 && has_insect(level, x, y - 1) {
                    count += 1;
                }

                let has = has_insect(level, x, y);
                new_map << 1
                    | if !has && (count == 1 || count == 2) || has && count == 1 {
                        1
                    } else {
                        0
                    }
            });
            result.push(new_level);
        }
        result.push(0);
        result.push(0);
        result
    }

    #[test]
    fn test_2() {
        let map = parse_map(
            "....#\n\
             #..#.\n\
             #..##\n\
             ..#..\n\
             #...."
                .split('\n'),
        );
        let mut levels = vec![0, 0, map, 0, 0];
        for _ in 0..10 {
            levels = step_rec(&levels);
        }
        dbg!(&levels);
        assert_eq!(
            levels[levels.len() / 2],
            parse_map(
                ".#...\n\
                 .#.##\n\
                 .#...\n\
                 .....\n\
                 ....."
                    .split('\n')
            )
        );
    }

    #[test]
    fn part_2() {
        let map = parse_map(util::input(24));

        let mut levels = vec![0, 0, map, 0, 0];
        for _ in 0..200 {
            levels = step_rec(&levels);
        }
        let result: u32 = levels.iter().map(|level| level.count_ones()).sum();

        assert_eq!(Some(result), util::answer(24, 2));
    }
}
