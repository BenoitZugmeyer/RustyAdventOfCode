use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::io::{stdin, BufRead};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Team {
    Infection,
    ImmuneSystem,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Group {
    team: Team,
    index: usize,
    count: isize,
    hit_points: isize,
    weaknesses: HashSet<String>,
    immunities: HashSet<String>,
    damages: isize,
    technic: String,
    initiative: isize,
}

impl Group {
    fn effective_power(&self) -> isize {
        self.count * self.damages
    }

    fn damages_to(&self, other: &Self) -> isize {
        if other.immunities.contains(&self.technic) {
            0
        } else if other.weaknesses.contains(&self.technic) {
            self.effective_power() * 2
        } else {
            self.effective_power()
        }
    }
}

impl std::str::FromStr for Group {
    type Err = ();
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+) units each with (\d+) hit points (?:\((.*?)\) )?with an attack that does (\d+) (\w+) damage at initiative (\d+)").unwrap();
        }
        let caps = RE.captures(line).ok_or(())?;

        let mut weaknesses = HashSet::new();
        let mut immunities = HashSet::new();
        if let Some(wi) = caps.get(3) {
            for wi in wi.as_str().split("; ") {
                if wi.starts_with("immune to ") {
                    immunities.extend(wi[10..].split(", ").map(|s| s.into()));
                } else if wi.starts_with("weak to ") {
                    weaknesses.extend(wi[8..].split(", ").map(|s| s.into()));
                }
            }
        }

        Ok(Self {
            team: Team::ImmuneSystem,
            index: 0,
            count: caps.get(1).ok_or(())?.as_str().parse().map_err(|_| ())?,
            hit_points: caps.get(2).ok_or(())?.as_str().parse().map_err(|_| ())?,
            weaknesses,
            immunities,
            damages: caps.get(4).ok_or(())?.as_str().parse().map_err(|_| ())?,
            technic: caps.get(5).ok_or(())?.as_str().into(),
            initiative: caps.get(6).ok_or(())?.as_str().parse().map_err(|_| ())?,
        })
    }
}

fn run_battle(mut groups: Vec<Group>) -> Option<(Team, isize)> {
    loop {
        let mut remaining_groups: Vec<_> = groups
            .iter()
            .filter(|group| group.count > 0)
            .cloned()
            .collect();

        if remaining_groups
            .iter()
            .all(|group| group.team == remaining_groups[0].team)
        {
            return Some((
                remaining_groups[0].team,
                remaining_groups
                    .iter()
                    .map(|group| group.count)
                    .sum::<isize>(),
            ));
        }
        remaining_groups.sort_by_key(|group| (-group.effective_power(), -group.initiative));

        let mut attacks: Vec<(usize, usize)> = Vec::new();
        for group in &remaining_groups {
            if let Some(enemy) = remaining_groups
                .iter()
                .filter(|other| other.team != group.team)
                .filter(|enemy| {
                    attacks
                        .iter()
                        .all(|(_, defending)| *defending != enemy.index)
                })
                .filter(|enemy| group.damages_to(enemy) > 0)
                .max_by_key(|enemy| {
                    (
                        group.damages_to(enemy),
                        enemy.effective_power(),
                        enemy.initiative,
                    )
                })
            {
                attacks.push((group.index, enemy.index));
            }
        }

        attacks.sort_by_key(|&(attacker, _)| -groups[attacker].initiative);

        let mut total_unit_loss = 0;
        for (attacker, defender) in attacks {
            let damages = groups[attacker].damages_to(&groups[defender]);
            if damages > 0 {
                let unit_loss = (damages / groups[defender].hit_points).min(groups[defender].count);
                total_unit_loss += unit_loss;
                groups[defender].count -= unit_loss;
            }
        }
        if total_unit_loss == 0 {
            return None;
        }
    }
}

fn main() {
    let lines: Vec<_> = stdin().lock().lines().filter_map(|l| l.ok()).collect();

    let mut groups = Vec::new();
    let mut team = Team::ImmuneSystem;

    for line in &lines {
        if line == "Immune System:" {
            team = Team::ImmuneSystem;
        } else if line == "Infection:" {
            team = Team::Infection;
        } else if let Ok(mut group) = line.parse::<Group>() {
            group.team = team;
            group.index = groups.len();
            groups.push(group);
        }
    }

    println!("Part 1: {}", run_battle(groups.clone()).unwrap().1);

    for boost in 1.. {
        if let Some((winning_team, units_count)) = run_battle(
            groups
                .iter()
                .cloned()
                .map(|mut group| {
                    if group.team == Team::ImmuneSystem {
                        group.damages += boost
                    }
                    group
                })
                .collect(),
        ) {
            if winning_team == Team::ImmuneSystem {
                println!("Part 2: {}", units_count);
                break;
            }
        }
    }
}
