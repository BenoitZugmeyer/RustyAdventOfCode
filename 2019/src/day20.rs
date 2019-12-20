#[cfg(test)]
mod tests {
    use crate::point::Point;
    use crate::util;
    use std::collections::HashMap;
    use std::convert::TryInto;

    type Portal = (char, char);
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    enum PortalType {
        Outer,
        Inner,
    }

    impl PortalType {
        fn other(self) -> Self {
            match self {
                Self::Outer => Self::Inner,
                Self::Inner => Self::Outer,
            }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    enum Tile {
        Path,
        Portal(Portal, PortalType),
    }

    type Map = HashMap<Point, Tile>;

    fn parse_map<S: AsRef<str>, I: Iterator<Item = S>>(iterator: I) -> Map {
        let char_map: Vec<Vec<char>> = iterator.map(|row| row.as_ref().chars().collect()).collect();

        let mut map = HashMap::new();
        for (y, row) in char_map.iter().enumerate() {
            for (x, ch) in row.iter().enumerate() {
                match ch {
                    ch @ 'A'..='Z' => {
                        let portal_type =
                            if x <= 2 || x >= row.len() - 3 || y <= 2 || y >= char_map.len() - 3 {
                                PortalType::Outer
                            } else {
                                PortalType::Inner
                            };

                        if x < row.len() - 1 && row[x + 1].is_ascii_uppercase() {
                            let px = if x == 0 || row[x - 1] != '.' {
                                x + 1
                            } else {
                                x
                            };
                            map.insert(
                                Point::new(px.try_into().unwrap(), y.try_into().unwrap()),
                                Tile::Portal((*ch, row[x + 1]), portal_type),
                            );
                        } else if y < char_map.len() - 1 && char_map[y + 1][x].is_ascii_uppercase()
                        {
                            let py = if y == 0 || char_map[y - 1][x] != '.' {
                                y + 1
                            } else {
                                y
                            };
                            map.insert(
                                Point::new(x.try_into().unwrap(), py.try_into().unwrap()),
                                Tile::Portal((*ch, char_map[y + 1][x]), portal_type),
                            );
                        }
                    }
                    '.' => {
                        map.insert(
                            Point::new(x.try_into().unwrap(), y.try_into().unwrap()),
                            Tile::Path,
                        );
                    }
                    _ => {}
                }
            }
        }

        map
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

    fn find_portal_positions<'a>(map: &'a Map, portal: Portal) -> impl Iterator<Item = Point> + 'a {
        map.iter()
            .filter_map(move |(key, tile)| {
                if let Tile::Portal(other, _) = tile {
                    if *other == portal {
                        Some(key)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .cloned()
    }

    fn find_shortest_path_rec(
        map: &Map,
        position: Point,
        steps: u32,
        discovered: &mut HashMap<Point, u32>,
    ) -> Option<u32> {
        let tile = match map.get(&position) {
            None => return None,
            Some(t) => t,
        };
        if !should_continue(position, steps, discovered) {
            return None;
        }

        let (position, steps) = match tile {
            Tile::Portal(('Z', 'Z'), _) => return Some(steps - 1),
            Tile::Portal(('A', 'A'), _) => (position, steps),
            Tile::Portal(portal, _) => {
                let output = find_portal_positions(map, *portal)
                    .find(|p| *p != position)
                    .unwrap();
                (output, steps)
            }
            Tile::Path => (position, steps + 1),
        };
        position
            .iter_nearby()
            .filter_map(|new_position| find_shortest_path_rec(map, new_position, steps, discovered))
            .min()
    }

    fn find_shortest_path(map: &Map) -> Option<u32> {
        let start = find_portal_positions(&map, ('A', 'A')).next().unwrap();
        find_shortest_path_rec(&map, start, 0, &mut HashMap::new())
    }

    fn discover_relations_rec(
        map: &Map,
        previous_direction: Point,
        position: Point,
        steps: u32,
        portals: &mut Vec<(Portal, PortalType, u32)>,
    ) {
        match map.get(&position) {
            Some(Tile::Portal(portal, portal_type)) => {
                portals.push((*portal, *portal_type, steps));
            }
            Some(Tile::Path) => {
                for next_position in position.iter_nearby() {
                    if next_position != previous_direction {
                        discover_relations_rec(map, position, next_position, steps + 1, portals);
                    }
                }
            }
            None => {}
        }
    }

    type Relations = HashMap<(Portal, PortalType), Vec<(Portal, PortalType, u32)>>;
    fn discover_relations(map: &Map) -> Relations {
        map.iter()
            .filter_map(|(position, tile)| match tile {
                Tile::Portal(portal, portal_type) => {
                    let next_position = position
                        .iter_nearby()
                        .find(|other| map.get(other).cloned() == Some(Tile::Path))
                        .unwrap();
                    let mut portals = Vec::new();
                    discover_relations_rec(map, *position, next_position, 0, &mut portals);
                    portals.sort_by_key(|(_, portal_type, _)| {
                        if *portal_type == PortalType::Outer {
                            0
                        } else {
                            1
                        }
                    });
                    Some(((*portal, *portal_type), portals))
                }
                _ => None,
            })
            .collect()
    }

    fn find_shortest_path_rec3(
        relations: &Relations,
        portal: Portal,
        portal_type: PortalType,
        steps: u32,
        level: u32,
        discovered: &mut HashMap<(Portal, PortalType, u32), u32>,
    ) -> Option<u32> {
        if !should_continue((portal, portal_type, level), steps, discovered) {
            return None;
        }

        let rel = match relations.get(&(portal, portal_type)) {
            Some(rel) => rel,
            None => return None,
        };

        let mut min: Option<u32> = None;
        for (output_portal, output_portal_type, cost) in rel {
            if level == 0 && *output_portal_type == PortalType::Outer {
                if output_portal == &('Z', 'Z') {
                    return Some(steps + cost - 1);
                }
            } else {
                let level = match output_portal_type {
                    PortalType::Outer => level - 1,
                    PortalType::Inner => level + 1,
                };
                if level > 1000 {
                    return None;
                }
                if steps > 50000 {
                    return None;
                }
                if let Some(r) = find_shortest_path_rec3(
                    relations,
                    *output_portal,
                    output_portal_type.other(),
                    steps + cost,
                    level,
                    discovered,
                ) {
                    min = Some(match min {
                        Some(min) => min.min(r),
                        None => r,
                    });
                }
            }
        }
        min
    }

    fn find_shortest_path2(map: &Map) -> Option<u32> {
        let relations = discover_relations(map);
        find_shortest_path_rec3(
            &relations,
            ('A', 'A'),
            PortalType::Outer,
            0,
            0,
            &mut HashMap::new(),
        )
    }

    #[test]
    fn test_1_1() {
        let map = parse_map(
            "#        A         \n\
             #        A         \n\
             #########.#########\n\
             #########.........#\n\
             #########.#######.#\n\
             #########.#######.#\n\
             #########.#######.#\n\
             #######  B    ###.#\n\
             BC...##  C    ###.#\n\
             ####.##       ###.#\n\
             ####...DE  F  ###.#\n\
             #######    G  ###.#\n\
             ###########.#####.#\n\
             DE..#######...###.#\n\
             ###.#########.###.#\n\
             FG..#########.....#\n\
             #############.#####\n\
             #            Z     \n\
             #            Z     "
                .split('\n'),
        );
        assert_eq!(find_shortest_path(&map), Some(23));
    }

    #[test]
    fn test_1_2() {
        let map = parse_map(
            "#                   A               \n\
             #                   A               \n\
             #  #################.#############  \n\
             #  #.#...#...................#.#.#  \n\
             #  #.#.#.###.###.###.#########.#.#  \n\
             #  #.#.#.......#...#.....#.#.#...#  \n\
             #  #.#########.###.#####.#.#.###.#  \n\
             #  #.............#.#.....#.......#  \n\
             #  ###.###########.###.#####.#.#.#  \n\
             #  #.....#        A   C    #.#.#.#  \n\
             #  #######        S   P    #####.#  \n\
             #  #.#...#                 #......VT\n\
             #  #.#.#.#                 #.#####  \n\
             #  #...#.#               YN....#.#  \n\
             #  #.###.#                 #####.#  \n\
             #DI....#.#                 #.....#  \n\
             #  #####.#                 #.###.#  \n\
             #ZZ......#               QG....#..AS\n\
             #  ###.###                 #######  \n\
             #JO..#.#.#                 #.....#  \n\
             #  #.#.#.#                 ###.#.#  \n\
             #  #...#..DI             BU....#..LF\n\
             #  #####.#                 #.#####  \n\
             #YN......#               VT..#....QG\n\
             #  #.###.#                 #.###.#  \n\
             #  #.#...#                 #.....#  \n\
             #  ###.###    J L     J    #.#.###  \n\
             #  #.....#    O F     P    #.#...#  \n\
             #  #.###.#####.#.#####.#####.###.#  \n\
             #  #...#.#.#...#.....#.....#.#...#  \n\
             #  #.#####.###.###.#.#.#########.#  \n\
             #  #...#.#.....#...#.#.#.#.....#.#  \n\
             #  #.###.#####.###.###.#.#.#######  \n\
             #  #.#.........#...#.............#  \n\
             #  #########.###.###.#############  \n\
             #           B   J   C               \n\
             #           U   P   P               "
                .split('\n'),
        );
        assert_eq!(find_shortest_path(&map), Some(58));
    }

    #[test]
    fn part_1() {
        let map = parse_map(util::input(20));
        assert_eq!(find_shortest_path(&map), util::answer(20, 1));
    }

    #[test]
    fn test_2_1() {
        let map = parse_map(
            "#        A         \n\
             #        A         \n\
             #########.#########\n\
             #########.........#\n\
             #########.#######.#\n\
             #########.#######.#\n\
             #########.#######.#\n\
             #######  B    ###.#\n\
             BC...##  C    ###.#\n\
             ####.##       ###.#\n\
             ####...DE  F  ###.#\n\
             #######    G  ###.#\n\
             ###########.#####.#\n\
             DE..#######...###.#\n\
             ###.#########.###.#\n\
             FG..#########.....#\n\
             #############.#####\n\
             #            Z     \n\
             #            Z     "
                .split('\n'),
        );
        assert_eq!(find_shortest_path2(&map), Some(26));
    }

    #[test]
    fn test_2_2() {
        let map = parse_map(
            "#             Z L X W       C                 \n\
             #             Z P Q B       K                 \n\
             #  ###########.#.#.#.#######.###############  \n\
             #  #...#.......#.#.......#.#.......#.#.#...#  \n\
             #  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  \n\
             #  #.#...#.#.#...#.#.#...#...#...#.#.......#  \n\
             #  #.###.#######.###.###.#.###.###.#.#######  \n\
             #  #...#.......#.#...#...#.............#...#  \n\
             #  #.#########.#######.#.#######.#######.###  \n\
             #  #...#.#    F       R I       Z    #.#.#.#  \n\
             #  #.###.#    D       E C       H    #.#.#.#  \n\
             #  #.#...#                           #...#.#  \n\
             #  #.###.#                           #.###.#  \n\
             #  #.#....OA                       WB..#.#..ZH\n\
             #  #.###.#                           #.#.#.#  \n\
             #CJ......#                           #.....#  \n\
             #  #######                           #######  \n\
             #  #.#....CK                         #......IC\n\
             #  #.###.#                           #.###.#  \n\
             #  #.....#                           #...#.#  \n\
             #  ###.###                           #.#.#.#  \n\
             #XF....#.#                         RF..#.#.#  \n\
             #  #####.#                           #######  \n\
             #  #......CJ                       NM..#...#  \n\
             #  ###.#.#                           #.###.#  \n\
             #RE....#.#                           #......RF\n\
             #  ###.###        X   X       L      #.#.#.#  \n\
             #  #.....#        F   Q       P      #.#.#.#  \n\
             #  ###.###########.###.#######.#########.###  \n\
             #  #.....#...#.....#.......#...#.....#.#...#  \n\
             #  #####.#.###.#######.#######.###.###.#.#.#  \n\
             #  #.......#.......#.#.#.#.#...#...#...#.#.#  \n\
             #  #####.###.#####.#.#.#.#.###.###.#.###.###  \n\
             #  #.......#.....#.#...#...............#...#  \n\
             #  #############.#.#.###.###################  \n\
             #               A O F   N                     \n\
             #               A A D   M                     "
                .split('\n'),
        );
        assert_eq!(find_shortest_path2(&map), Some(396));
    }

    #[test]
    fn part_2() {
        let map = parse_map(util::input(20));
        assert_eq!(find_shortest_path2(&map), util::answer(20, 2));
    }
}
