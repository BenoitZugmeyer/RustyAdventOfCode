use std::collections::HashMap;

type Chemical = usize;
type ReactionsSlice = [(u64, Vec<(Chemical, u64)>)];
const FUEL: usize = 0;
const ORE: usize = 10000;

fn run_reactions(reactions: &ReactionsSlice, overproduced_chemicals: &mut [u64], needed_fuel: u64) -> Option<u64> {
    let mut missing_chemicals = HashMap::new();
    missing_chemicals.insert(FUEL, needed_fuel);

    while missing_chemicals.len() > 1 || !missing_chemicals.contains_key(&ORE) {
        let chemical = missing_chemicals
            .keys()
            .find(|k| k != &&ORE)
            .cloned()
            .unwrap();
        let mut quantity_needed = missing_chemicals.remove(&chemical).unwrap();

        let mut overproduced = overproduced_chemicals[chemical];
        if overproduced > 0 {
            if overproduced > quantity_needed {
                overproduced -= quantity_needed;
                quantity_needed = 0;
            } else {
                quantity_needed -= overproduced;
                overproduced = 0;
            }
            overproduced_chemicals[chemical] = overproduced;
        }

        if quantity_needed > 0 {
            let (quantity_produced, inputs) = &reactions[chemical];
            let factor = if quantity_needed % quantity_produced == 0 {
                quantity_needed / quantity_produced
            } else {
                quantity_needed / quantity_produced + 1
            };
            let final_quantity_produced = quantity_produced * factor;
            overproduced_chemicals[chemical] = final_quantity_produced - quantity_needed;

            for (chemical, quantity) in inputs {
                *missing_chemicals.entry(*chemical).or_insert(0) += quantity * factor;
            }
        }
    }
    missing_chemicals.get(&ORE).cloned()
}

#[allow(dead_code)]
fn run_reactions_for_one_fuel(reactions: &ReactionsSlice) -> Option<u64> {
    run_reactions(reactions, &mut vec![0; reactions.len()], 1)
}

#[allow(
    dead_code,
    clippy::cast_lossless,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation
)]
fn run_reactions_for_ore_mildly_optimized(reactions: &ReactionsSlice, ore: u64) -> Option<u64> {
    let mut overproduced_chemicals = vec![0; reactions.len()];
    let mut required_ore = 0;
    let mut fuel = 0;
    while required_ore < ore {
        required_ore += run_reactions(reactions, &mut overproduced_chemicals, 1).unwrap();
        fuel += 1;
        if overproduced_chemicals.iter().all(|&q| q == 0) {
            break;
        }
    }

    Some((ore as f64 / required_ore as f64 * fuel as f64) as u64)
}

#[allow(dead_code)]
fn run_reactions_for_ore(reactions: &ReactionsSlice, ore: u64) -> Option<u64> {
    let mut steps = 1_000_000;
    let mut fuel = steps;
    loop {
        let required_ore = run_reactions(reactions, &mut vec![0; reactions.len()], fuel).unwrap();
        if required_ore > ore {
            if steps == 1 {
                return Some(fuel - 1);
            }
            fuel = fuel - steps + steps / 10;
            steps /= 10;
        } else {
            fuel += steps;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;
    use itertools::Itertools;
    type Reactions = Vec<(u64, Vec<(Chemical, u64)>)>;

    fn parse_quantity(chemical_ids: &mut HashMap<String, usize>, s: &str) -> (Chemical, u64) {
        let (quantity, chemical) = s.split(' ').tuples().next().unwrap();
        let next_id = chemical_ids.len() - 1;
        let chemical_id = chemical_ids.entry(chemical.to_string()).or_insert(next_id);
        (*chemical_id, quantity.parse().unwrap())
    }

    fn parse_reactions<S: AsRef<str>, I: Iterator<Item = S>>(iterator: I) -> Reactions {
        let mut chemical_ids: HashMap<String, usize> = HashMap::new();
        chemical_ids.insert("FUEL".into(), FUEL);
        chemical_ids.insert("ORE".into(), ORE);

        let hash: HashMap<_, _> = iterator
            .map(|line| {
                let (input, output) = line.as_ref().split(" => ").tuples().next().unwrap();

                let inputs: Vec<_> = input
                    .split(", ")
                    .map(|s| parse_quantity(&mut chemical_ids, s))
                    .collect();
                let (chemical, quantity) = parse_quantity(&mut chemical_ids, output);
                (chemical, (quantity, inputs))
            })
            .collect();

        (0..hash.len()).map(|i| hash[&i].clone()).collect()
    }

    fn parse_reactions_from_str(s: &str) -> Reactions {
        parse_reactions(s.split('\n'))
    }

    #[test]
    fn test_1_1() {
        let reactions = parse_reactions_from_str(
            "10 ORE => 10 A\n\
             1 ORE => 1 B\n\
             7 A, 1 B => 1 C\n\
             7 A, 1 C => 1 D\n\
             7 A, 1 D => 1 E\n\
             7 A, 1 E => 1 FUEL",
        );
        assert_eq!(run_reactions_for_one_fuel(&reactions), Some(31));
    }

    #[test]
    fn test_1_2() {
        let reactions = parse_reactions_from_str(
            "9 ORE => 2 A\n\
             8 ORE => 3 B\n\
             7 ORE => 5 C\n\
             3 A, 4 B => 1 AB\n\
             5 B, 7 C => 1 BC\n\
             4 C, 1 A => 1 CA\n\
             2 AB, 3 BC, 4 CA => 1 FUEL",
        );
        assert_eq!(run_reactions_for_one_fuel(&reactions), Some(165));
    }

    #[test]
    fn test_1_3() {
        let reactions = parse_reactions_from_str(
            "157 ORE => 5 NZVS\n\
             165 ORE => 6 DCFZ\n\
             44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n\
             12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n\
             179 ORE => 7 PSHF\n\
             177 ORE => 5 HKGWZ\n\
             7 DCFZ, 7 PSHF => 2 XJWVT\n\
             165 ORE => 2 GPVTF\n\
             3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
        );
        assert_eq!(run_reactions_for_one_fuel(&reactions), Some(13312));
    }

    #[test]
    fn part_1() {
        let reactions: Reactions = parse_reactions(util::input(14));
        assert_eq!(run_reactions_for_one_fuel(&reactions), util::answer(14, 1));
    }

    #[test]
    fn test_2_1() {
        let reactions = parse_reactions_from_str(
            "157 ORE => 5 NZVS\n\
             165 ORE => 6 DCFZ\n\
             44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n\
             12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n\
             179 ORE => 7 PSHF\n\
             177 ORE => 5 HKGWZ\n\
             7 DCFZ, 7 PSHF => 2 XJWVT\n\
             165 ORE => 2 GPVTF\n\
             3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
        );

        assert_eq!(
            run_reactions_for_ore_mildly_optimized(&reactions, 1_000_000_000_000),
            Some(82_892_753)
        );
    }

    #[test]
    fn test_2_2() {
        let reactions = parse_reactions_from_str(
            "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG\n\
             17 NVRVD, 3 JNWZP => 8 VPVL\n\
             53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL\n\
             22 VJHF, 37 MNCFX => 5 FWMGM\n\
             139 ORE => 4 NVRVD\n\
             144 ORE => 7 JNWZP\n\
             5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC\n\
             5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV\n\
             145 ORE => 6 MNCFX\n\
             1 NVRVD => 8 CXFTF\n\
             1 VJHF, 6 MNCFX => 4 RFSQX\n\
             176 ORE => 6 VJHF",
        );

        assert_eq!(
            run_reactions_for_ore(&reactions, 1_000_000_000_000),
            Some(5_586_022)
        );
    }

    #[test]
    fn test_2_3() {
        let reactions = parse_reactions_from_str(
            "171 ORE => 8 CNZTR\n\
             7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL\n\
             114 ORE => 4 BHXH\n\
             14 VRPVC => 6 BMBT\n\
             6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL\n\
             6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT\n\
             15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW\n\
             13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW\n\
             5 BMBT => 4 WPTQ\n\
             189 ORE => 9 KTJDG\n\
             1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP\n\
             12 VRPVC, 27 CNZTR => 2 XDBXC\n\
             15 KTJDG, 12 BHXH => 5 XCVML\n\
             3 BHXH, 2 VRPVC => 7 MZWV\n\
             121 ORE => 7 VRPVC\n\
             7 XCVML => 6 RJRHP\n\
             5 BHXH, 4 VRPVC => 5 LTCX",
        );

        assert_eq!(
            run_reactions_for_ore(&reactions, 1_000_000_000_000),
            Some(460_664)
        );
    }

    #[test]
    fn part_2() {
        let reactions: Reactions = parse_reactions(util::input(14));
        assert_eq!(
            run_reactions_for_ore(&reactions, 1_000_000_000_000),
            util::answer(14, 2)
        );
    }
}
