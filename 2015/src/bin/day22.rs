use std::io;
use std::io::BufRead;
use std::cmp::max;

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

#[derive(Debug, Default, Clone)]
struct Spell {
    name: String,
    cost: u8,
    turns: u8,

    armor: u8,
    damages: u8,
    heal: u8,
    mana: u8,
}

impl Spell {
    fn get_availables() -> Vec<Self> {
        vec![
            Spell { name: "Magic Missile".to_string(), cost: 53, turns: 1, damages: 4, ..Spell::default() },
            Spell { name: "Drain".to_string(), cost: 73, turns: 1, damages: 2, heal: 2, ..Spell::default() },
            Spell { name: "Shield".to_string(), cost: 113, turns: 6, armor: 7, ..Spell::default() },
            Spell { name: "Poison".to_string(), cost: 173, turns: 6, damages: 3, ..Spell::default() },
            Spell { name: "Recharge".to_string(), cost: 229, turns: 5, mana: 101, ..Spell::default() },
        ]
    }
}

#[derive(Debug, Default, Clone)]
struct Character {
    hp: i32,
    damage: i32,
    armor: i32,
    mp: i32,
}

impl Character {
}

#[derive(Debug, Clone)]
struct State<'a> {
    wizard_hp: i32,
    wizard_mp: i32,
    wizard_armor: i32,
    boss_hp: i32,
    active_spells: Vec<u8>,
    world: &'a World<'a>,
}

impl<'a> State<'a> {
    fn new(world: &'a World) -> Self {
        State {
            wizard_hp: world.wizard.hp,
            wizard_mp: world.wizard.mp,
            wizard_armor: world.wizard.armor,
            boss_hp: world.boss.hp,
            active_spells: vec![0; world.spells.len()],
            world: world,
        }
    }

    fn apply_spells(&mut self) {
        self.wizard_armor = self.world.wizard.armor;

        for (spell, turns_remaining) in self.world.spells.iter().zip(self.active_spells.iter_mut()) {
            if *turns_remaining == 0 { continue }
            *turns_remaining -= 1;

            self.boss_hp -= spell.damages as i32;
            self.wizard_armor += spell.armor as i32;
            self.wizard_hp += spell.heal as i32;
            self.wizard_mp += spell.mana as i32;
        }
    }

    fn cast_spell(&mut self, spell_index: usize) -> Option<u8> {
        if self.active_spells[spell_index] > 0 { return None }
        let spell = &self.world.spells[spell_index];
        if self.wizard_mp < spell.cost as i32 { return None }
        self.wizard_mp -= spell.cost as i32;
        self.active_spells[spell_index] = spell.turns;
        Some(spell.cost)
    }
}

#[derive(Debug)]
struct World<'a> {
    wizard: &'a Character,
    boss: &'a Character,
    spells: Vec<Spell>,
    hard: bool,
}

fn fight_round(cast_spell_index: usize, mut state: State) -> Option<u32> {

    if state.world.hard {
        state.wizard_hp -= 1;

        if state.wizard_hp <= 0 {
            return None
        }
    }

    // Wizard spells
    state.apply_spells();

    // Boss dies
    if state.boss_hp <= 0 {
        return Some(0)
    }

    let cost = if let Some(cost) = state.cast_spell(cast_spell_index) {
        cost as u32
    }
    else {
        // Not enough mana
        return None
    };

    // Wizard spells
    state.apply_spells();

    // Boss dies
    if state.boss_hp <= 0 {
        return Some(cost)
    }

    // Boss hits
    state.wizard_hp -= max(1, state.world.boss.damage - state.wizard_armor);

    // Wizard dies
    if state.wizard_hp <= 0 {
        return None
    }

    fight(&state).map(|c| c + cost)
}

fn fight(state: &State) -> Option<u32> {
    (0..state.world.spells.len())
        .filter_map(|index| fight_round(index, state.clone()))
        .min()
}

#[test]
fn example1() {
    let world = World {
        wizard: &Character { hp: 10, mp: 250, ..Character::default() },
        boss: &Character { hp: 13, damage: 8, ..Character::default() },
        spells: Spell::get_availables(),
        hard: false,
    };

    assert_eq!(fight(&State::new(&world)), Some(226));
}

#[test]
fn example2() {
    let world = World {
        wizard: &Character { hp: 10, mp: 250, ..Character::default() },
        boss: &Character { hp: 14, damage: 8, ..Character::default() },
        spells: Spell::get_availables(),
        hard: false,
    };

    assert_eq!(fight(&State::new(&world)), Some(641));
}

fn main() {
    let stdin = io::stdin();

    let mut boss = Character::default();

    for line in stdin.lock().lines().filter_map(|l| l.ok()) {
        if let Some((key, value)) = line.partition(": ") {
            if let Ok(value) = value.parse::<i32>() {
                match key {
                    "Hit Points" => boss.hp = value,
                    "Damage" => boss.damage = value,
                    _ => {},
                }
            }
        }
    }

    let wizard = Character { hp: 50, mp: 500, ..Character::default() };

    let world = World {
        boss: &boss,
        wizard: &wizard,
        spells: Spell::get_availables(),
        hard: false,
    };

    if let Some(result) = fight(&State::new(&world)) {
        println!("Minimum mana on easy mode: {}", result);
    }
    else {
        println!("Can't win on easy mode :(");
    }

    let world = World {
        boss: &boss,
        wizard: &wizard,
        spells: Spell::get_availables(),
        hard: true,
    };

    if let Some(result) = fight(&State::new(&world)) {
        println!("Minimum mana on hard mode: {}", result);
    }
    else {
        println!("Can't win on hard mode :(");
    }
}
