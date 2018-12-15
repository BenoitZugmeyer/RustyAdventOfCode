use std::borrow::Borrow;
use std::cell;
use std::collections::{BTreeMap, HashSet};
use std::io::{stdin, BufRead};
use std::rc::Rc;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

struct Directions {
    index: usize,
}

impl Directions {
    fn new() -> Self {
        Self { index: 0 }
    }
}
impl Iterator for Directions {
    type Item = Direction;
    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        match self.index {
            1 => Some(Direction::North),
            2 => Some(Direction::West),
            3 => Some(Direction::East),
            4 => Some(Direction::South),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coordinates {
    x: usize,
    y: usize,
}

impl std::cmp::Ord for Coordinates {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.y, self.x).cmp(&(other.y, other.x))
    }
}
impl std::cmp::PartialOrd for Coordinates {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Coordinates {
    fn next(&self, direction: Direction) -> Self {
        match direction {
            Direction::North => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::East => Self {
                x: self.x + 1,
                y: self.y,
            },
            Direction::West => Self {
                x: self.x - 1,
                y: self.y,
            },
            Direction::South => Self {
                x: self.x,
                y: self.y + 1,
            },
        }
    }

    fn nearby(self) -> impl Iterator<Item = Self> {
        Directions::new().map(move |direction| self.next(direction))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum UnitKind {
    Goblin,
    Elf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Unit {
    kind: UnitKind,
    hp: cell::Cell<i64>,
    coordinates: cell::Cell<Coordinates>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Cell {
    Wall,
    Open,
}

struct Map(Vec<Vec<Cell>>);

struct Runner<'a> {
    map: &'a Map,
    units: BTreeMap<Coordinates, Rc<Unit>>,
    elf_attack_power: i64,
    elves_count: usize,
}

impl<'a> Runner<'a> {
    fn new(map: &'a Map, units: &[Unit], elf_attack_power: i64) -> Self {
        Self {
            map,
            units: units
                .iter()
                .map(|unit| (unit.coordinates.get(), Rc::new(unit.clone())))
                .collect(),
            elf_attack_power,
            elves_count: units.iter().filter(|u| u.kind == UnitKind::Elf).count(),
        }
    }

    fn format(&self, markers: Option<&Vec<Coordinates>>) -> String {
        let mut result = String::new();
        for (y, row) in self.map.0.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let coord = Coordinates { x, y };
                result.push(match cell {
                    Cell::Open => markers
                        .as_ref()
                        .and_then(|coordinates| {
                            if coordinates.contains(&coord) {
                                Some('!')
                            } else {
                                None
                            }
                        })
                        .unwrap_or_else(|| match self.units.get(&coord).map(|rc| rc.borrow()) {
                            Some(Unit {
                                kind: UnitKind::Elf,
                                ..
                            }) => 'E',
                            Some(Unit {
                                kind: UnitKind::Goblin,
                                ..
                            }) => 'G',
                            None => '.',
                        }),
                    Cell::Wall => '#',
                });
            }
            result.push('\n');
        }

        result
    }

    #[allow(dead_code)]
    fn print(&self, markers: Option<&Vec<Coordinates>>) {
        println!("{}", self.format(markers));
    }

    fn full_combat(&mut self) -> i64 {
        let mut round = 0;
        while self.round() {
            round += 1;
        }
        round
    }

    fn full_combat_no_dead_elf(&mut self) -> Option<i64> {
        let initial_elves_count = self.elves_count;
        let mut round = 0;
        while self.round() {
            if initial_elves_count != self.elves_count {
                return None;
            }
            round += 1;
        }
        if initial_elves_count != self.elves_count {
            return None;
        }
        Some(round)
    }

    fn round(&mut self) -> bool {
        let units: Vec<_> = self.units.values().cloned().collect();

        for unit in &units {
            if unit.hp.get() <= 0 {
                continue;
            }

            if unit.kind == UnitKind::Goblin && self.elves_count == 0 {
                return false;
            }

            if unit.kind == UnitKind::Elf && self.elves_count == self.units.len() {
                return false;
            }

            if let Some((root_coords, distance)) = self.find_target_enemy(unit) {
                // If a target has been found, go toward it (if it is not nearby)
                if distance > 0 {
                    let unit = self.units.remove(&unit.coordinates.get()).unwrap();
                    unit.coordinates.set(root_coords);
                    self.units.insert(root_coords, unit);
                }

                // And chose if we should attack or not
                if distance <= 1 {
                    let enemy = unit
                        .coordinates
                        .get()
                        .nearby()
                        .filter_map(|coords| self.units.get(&coords))
                        .filter(|other| other.kind != unit.kind)
                        .min_by_key(|enemy| enemy.hp.get())
                        .unwrap();

                    let new_hp = enemy.hp.get()
                        - if unit.kind == UnitKind::Elf {
                            self.elf_attack_power
                        } else {
                            3
                        };
                    enemy.hp.set(new_hp);

                    if new_hp <= 0 {
                        let coords = enemy.coordinates.get();
                        if enemy.kind == UnitKind::Elf {
                            self.elves_count -= 1;
                        }
                        self.units.remove(&coords);
                    }
                };
            }
        }

        true
    }

    fn get_cell_at(&self, coordinates: &Coordinates) -> Cell {
        self.map.0[coordinates.y][coordinates.x].clone()
    }

    fn find_target_enemy(&self, unit: &Rc<Unit>) -> Option<(Coordinates, usize)> {
        let mut passed_coordinates: HashSet<Coordinates> = HashSet::new();

        let mut edge_coordinates: Vec<(Coordinates, Coordinates)> = unit
            .coordinates
            .get()
            .nearby()
            .map(|coords| (coords, coords))
            .collect();

        let mut target: Option<(Coordinates, Coordinates)> = None;

        let mut distance = 0;

        while target.is_none() && !edge_coordinates.is_empty() {
            let mut next_edge_coordinates: Vec<(Coordinates, Coordinates)> = Vec::new();

            for (edge_coords, root_coords) in &edge_coordinates {
                // We already handled those coordinates
                if !passed_coordinates.insert(*edge_coords) {
                    continue;
                }

                // The cell at this coordinates is not empty
                if self.get_cell_at(&edge_coords) != Cell::Open {
                    continue;
                }

                // Find the enemy at those coordinates
                if let Some(Unit { kind, .. }) = self.units.get(&edge_coords).map(|rc| rc.borrow())
                {
                    if *kind == unit.kind {
                        continue;
                    } else {
                        let new_coords = (*edge_coords, *root_coords);
                        target = target
                            // If we already found an enemy at this distance, keep the first one
                            // according (coordinates order)
                            .map(|h| std::cmp::min(h, new_coords))
                            // Else store the new enemy found
                            .or_else(|| Some(new_coords));
                    }
                }

                if target.is_none() {
                    next_edge_coordinates.extend(
                        edge_coords
                            .nearby()
                            .map(|coordinates| (coordinates, *root_coords)),
                    );
                }
            }

            // Consider cells at the distance + 1
            edge_coordinates = next_edge_coordinates;
            distance += 1;
        }

        target.map(|(_, root_coords)| (root_coords, distance - 1))
    }

    fn hp_sum(&self) -> i64 {
        self.units.values().map(|u| u.hp.get()).sum::<i64>()
    }
}

fn parse_map(lines: impl Iterator<Item = String>) -> (Map, Vec<Unit>) {
    let mut units = Vec::new();
    let map: Vec<Vec<Cell>> = lines
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, ch)| match ch {
                    '#' => Cell::Wall,
                    'G' | 'E' => {
                        units.push(Unit {
                            kind: match ch {
                                'G' => UnitKind::Goblin,
                                'E' => UnitKind::Elf,
                                _ => unreachable!(),
                            },
                            hp: cell::Cell::new(200),
                            coordinates: cell::Cell::new(Coordinates { x, y }),
                        });
                        Cell::Open
                    }
                    _ => Cell::Open,
                })
                .collect()
        })
        .collect();
    (Map(map), units)
}

fn part_1(map: &Map, units: &[Unit]) -> i64 {
    let mut runner = Runner::new(map, units, 3);
    let round = runner.full_combat();
    round * runner.hp_sum()
}

fn part_2(map: &Map, units: &[Unit]) -> i64 {
    let (round, sum) = (3..)
        .filter_map(|elf_attack_power| {
            let mut runner = Runner::new(map, units, elf_attack_power);
            runner
                .full_combat_no_dead_elf()
                .map(|round| (round, runner.hp_sum()))
        })
        .next()
        .unwrap();
    round * sum
}

fn main() {
    let (map, units) = parse_map(stdin().lock().lines().filter_map(|l| l.ok()));

    println!("Part 1: {}", part_1(&map, &units));
    println!("Part 2: {}", part_2(&map, &units));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_map_string(input: &str) -> (Map, Vec<Unit>) {
        parse_map(input.split('\n').skip(1).map(|s| s.into()))
    }

    fn compare_round(from: &str, to: &str) {
        let (map, units) = parse_map_string(from);
        let mut runner = Runner::new(&map, &units, 3);
        runner.round();
        let (map, units) = parse_map_string(to);
        let runner2 = Runner::new(&map, &units, 3);
        assert_eq!(runner.format(None), runner2.format(None));
    }

    fn compare_find_target_enemy(from: &str, index: usize, result: Option<(Coordinates, usize)>) {
        let (map, units) = parse_map_string(from);
        let runner = Runner::new(&map, &units, 3);
        assert_eq!(
            runner.find_target_enemy(runner.units.values().nth(index).unwrap()),
            result
        );
    }

    fn test_full_combat(from: &str, round: i64, sum: i64) {
        let (map, units) = parse_map_string(from);
        let mut runner = Runner::new(&map, &units, 3);
        assert_eq!(round, runner.full_combat());
        assert_eq!(sum, runner.hp_sum());
    }

    fn test_full_combat_part_2(from: &str, expected_round: i64, expected_sum: i64) {
        let (map, units) = parse_map_string(from);
        assert_eq!(part_2(&map, &units), expected_round * expected_sum);
    }

    #[test]
    fn test_base_deplacement1() {
        compare_find_target_enemy(
            "
#######
#.E...#
#.....#
#...G.#
#######",
            0,
            Some((Coordinates { x: 3, y: 1 }, 3)),
        );
    }

    #[test]
    fn test_base_deplacement2() {
        compare_round(
            "
#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########",
            "
#########
#.G...G.#
#...G...#
#...E..G#
#.G.....#
#.......#
#G..G..G#
#.......#
#########",
        );
    }

    #[test]
    fn test_base_deplacement3() {
        compare_find_target_enemy(
            "
#########
#..G.G..#
#...G...#
#...E..G#
#.G.....#
#.......#
#G..G..G#
#.......#
#########",
            2,
            Some((Coordinates { x: 4, y: 3 }, 0)),
        );
    }

    #[test]
    fn test_base_deplacement4() {
        compare_round(
            "
#########
#.G...G.#
#...G...#
#...E..G#
#.G.....#
#.......#
#G..G..G#
#.......#
#########",
            "
#########
#..G.G..#
#...G...#
#.G.E.G.#
#.......#
#G..G..G#
#.......#
#.......#
#########",
        );
    }

    #[test]
    fn test_base_deplacement5() {
        compare_round(
            "
#########
#..G.G..#
#...G...#
#.G.E.G.#
#.......#
#G..G..G#
#.......#
#.......#
#########",
            "
#########
#.......#
#..GGG..#
#..GEG..#
#G..G...#
#......G#
#.......#
#.......#
#########",
        );
    }

    #[test]
    fn test_base_deplacement6() {
        compare_find_target_enemy(
            "
#######
#G.E..#
#.G...#
#.....#
#######",
            1,
            Some((Coordinates { x: 2, y: 1 }, 1)),
        );
    }

    #[test]
    fn test_full_combat1() {
        test_full_combat(
            "
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######",
            47,
            590,
        );
    }

    #[test]
    fn test_full_combat2() {
        test_full_combat(
            "
#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######",
            37,
            982,
        );
    }

    #[test]
    fn test_full_combat_part_2_1() {
        test_full_combat_part_2(
            "
    #######
    #.G...#
    #...EG#
    #.#.#G#
    #..G#E#
    #.....#
    #######",
            29,
            172,
        );
    }

    #[test]
    fn test_full_combat_part_2_2() {
        test_full_combat_part_2(
            "
    #######
    #E..EG#
    #.#G.E#
    #E.##E#
    #G..#.#
    #..E#.#
    #######",
            33,
            948,
        );
    }

    #[test]
    fn test_full_combat_part_2_3() {
        test_full_combat_part_2(
            "
    #######
    #E.G#.#
    #.#G..#
    #G.#.G#
    #G..#.#
    #...E.#
    #######",
            37,
            94,
        );
    }

    #[test]
    fn test_full_combat_part_2_4() {
        test_full_combat_part_2(
            "
    #######
    #.E...#
    #.#..G#
    #.###.#
    #E#G#G#
    #...#G#
    #######",
            39,
            166,
        );
    }

    #[test]
    fn test_full_combat_part_2_5() {
        test_full_combat_part_2(
            "
    #########
    #G......#
    #.E.#...#
    #..##..G#
    #...##..#
    #...#...#
    #.G...G.#
    #.....G.#
    #########",
            30,
            38,
        );
    }
    #[test]
    fn test_base_deplacement7() {
        compare_find_target_enemy(
            "
###########
#..G..#####
#......####
#.......###
#........##
#...###...#
##...#E...#
#...##..#.#
#..##.....#
##.E#.##..#
###....#.##
###########",
            0,
            Some((Coordinates { x: 4, y: 1 }, 9)),
        );
    }

    #[test]
    fn test_base_deplacement8() {
        compare_find_target_enemy(
            "
#######
#.E.EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######",
            3,
            Some((Coordinates { x: 3, y: 1 }, 1)),
        );
    }

}
