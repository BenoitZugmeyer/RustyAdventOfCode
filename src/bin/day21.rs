use std::io;
use std::io::BufRead;
use std::cmp::{min, max};

#[derive(Debug)]
struct Character {
    hp: i32,
    damage: i32,
    armor: i32,
}

impl Character {
    fn equip(&mut self, item: &Item) {
        self.damage += item.damage;
        self.armor += item.armor;
    }

    fn would_win_against(&self, ennemy: &Character) -> bool {
        let character_damage_per_turn = max(1, self.damage - ennemy.armor);
        let ennemy_damage_per_turn = max(1, ennemy.damage - self.armor);
        (ennemy.hp + character_damage_per_turn - 1) / character_damage_per_turn <=
            (self.hp + ennemy_damage_per_turn - 1) / ennemy_damage_per_turn
    }
}

impl Default for Character {
    fn default() -> Self {
        Character { hp: 0, damage: 0, armor: 0 }
    }
}

#[derive(Debug)]
struct Item {
    cost: i32,
    damage: i32,
    armor: i32,
}

impl Default for Item {
    fn default() -> Self {
        Item { cost: 0, damage: 0, armor: 0 }
    }
}


struct Permutations {
    index: usize,
    maxes: Vec<usize>,
    max: usize,
}

impl Permutations {
    fn new(maxes: Vec<usize>) -> Self {
        let max = maxes.iter().fold(1, |t, m| t * m);
        Permutations { maxes: maxes, index: 0, max: max }
    }
}

impl Iterator for Permutations {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut index = self.index;
        if index < self.max {
            self.index += 1;

            Some(self.maxes.iter().map(|len| {
                let value = index % len;
                index /= *len;
                value
            }).collect())
        }
        else {
            None
        }
    }
}

trait Partition {
    fn partition(&self, needle: &Self) -> Option<(&Self, &Self)>;
}

impl Partition for str {
    fn partition(&self, needle: &str) -> Option<(&str, &str)> {
        self.find(needle).map(|index| {
            let (key, value) = self.split_at(index);
            (key, &value[needle.len()..])
        })
    }
}

#[test]
fn would_win_against() {
    let boss = Character { hp: 12, damage: 7, armor: 2 };
    let you = Character { hp: 8, damage: 5, armor: 5 };
    assert!(you.would_win_against(&boss));
}

#[test]
fn would_win_against2() {
    let boss = Character { hp: 13, damage: 7, armor: 2 };
    let you = Character { hp: 8, damage: 5, armor: 5 };
    assert!(!you.would_win_against(&boss));
}


fn main() {
    let stdin = io::stdin();

    let mut ennemy = Character::default();

    for line in stdin.lock().lines().filter_map(|l| l.ok()) {
        if let Some((key, value)) = line.partition(": ") {
            if let Ok(value) = value.parse::<i32>() {
                match key {
                    "Hit Points" => ennemy.hp = value,
                    "Damage" => ennemy.damage = value,
                    "Armor" => ennemy.armor = value,
                    _ => {},
                }
            }
        }
    }

    let weapons = vec![
        Item { cost: 8, damage: 4, armor: 0 },
        Item { cost: 10, damage: 5, armor: 0 },
        Item { cost: 25, damage: 6, armor: 0 },
        Item { cost: 40, damage: 7, armor: 0 },
        Item { cost: 74, damage: 8, armor: 0 },
    ];

    let armors = vec![
        Item { cost: 13, damage: 0, armor: 1 },
        Item { cost: 31, damage: 0, armor: 2 },
        Item { cost: 53, damage: 0, armor: 3 },
        Item { cost: 75, damage: 0, armor: 4 },
        Item { cost: 102, damage: 0, armor: 5 },
    ];

    let rings = vec![
        Item { cost: 25, damage: 1, armor: 0 },
        Item { cost: 50, damage: 2, armor: 0 },
        Item { cost: 100, damage: 3, armor: 0 },
        Item { cost: 20, damage: 0, armor: 1 },
        Item { cost: 40, damage: 0, armor: 2 },
        Item { cost: 80, damage: 0, armor: 3 },
    ];

    let (min_cost, max_cost) = Permutations::new(vec![weapons.len(), armors.len() + 1, rings.len() + 1, rings.len() + 1])
        .filter_map(|indexes| {
            // Skip same ring
            if rings.len() != indexes[2] && indexes[2] == indexes[3] { return None }

            let stuff = [
                weapons.get(indexes[0]),
                armors.get(indexes[1]),
                rings.get(indexes[2]),
                rings.get(indexes[3]),
            ];

            let mut total_item = Item::default();
            for item in stuff.iter().filter_map(|i| *i) {
                total_item.cost += item.cost;
                total_item.damage += item.damage;
                total_item.armor += item.armor;
            }

            Some(total_item)
        })
        .fold((i32::max_value(), 0), |(pmin, pmax), ref total_item| {
            let mut character = Character::default();
            character.hp = 100;
            character.equip(total_item);
            if character.would_win_against(&ennemy) {
                (min(pmin, total_item.cost), pmax)
            }
            else {
                (pmin, max(pmax, total_item.cost))
            }
        });

    println!("Minimal stuff cost to beat the boss: {}", min_cost);
    println!("Maximal stuff cost to be defeated by the boss: {}", max_cost);
}
