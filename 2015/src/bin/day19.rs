#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::btree_map;
use std::collections::VecDeque;
use std::io;
use std::io::BufRead;


trait MapGetDefault<K, V> {
    fn get_default_mut(&mut self, key: K) -> &mut V;
}

impl<K: Ord, V: Default> MapGetDefault<K, V> for BTreeMap<K, V> {
    fn get_default_mut(&mut self, key: K) -> &mut V {
        self.entry(key).or_insert_with(V::default)
    }
}


type Molecule = u8;
type Mutations = BTreeMap<Molecule, Vec<Vec<Molecule>>>;

fn mutate(molecule: &[Molecule], mutations: &Mutations) -> BTreeSet<Vec<Molecule>> {
    let molecule_len = molecule.len();

    let mut mutated_molecules = BTreeSet::new();

    for (source, targets) in mutations {
        for (indice, _) in molecule.iter().enumerate().filter(|&(_, m)| m == source) {
            for target in targets {
                let (before, after) = molecule.split_at(indice);
                let target_len = target.len();

                let mut mutaded_molecule = Vec::with_capacity(molecule_len - 1 + target_len);
                mutaded_molecule.extend_from_slice(before);
                mutaded_molecule.extend_from_slice(&target);
                mutaded_molecule.extend_from_slice(&after[1..]);
                mutated_molecules.insert(mutaded_molecule);
            }
        }
    }

    mutated_molecules
}

struct SubSlicesIterator<'a, T: 'a> {
    position: usize,
    needle: &'a [T],
    slice: &'a [T],
}

impl<'a, T: 'a> SubSlicesIterator<'a, T> {
    fn new(slice: &'a[T], needle: &'a[T]) -> Self {
        SubSlicesIterator {
            position: 0,
            needle: needle,
            slice: slice,
        }
    }

    fn move_next(&mut self) {
        self.position += 1;
        self.slice = &self.slice[1..];
    }
}

impl<'a, T: 'a + PartialEq + std::fmt::Debug> Iterator for SubSlicesIterator<'a, T> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        while !self.slice.is_empty() {
            if self.slice.starts_with(self.needle) {
                let position = self.position;
                self.move_next();
                return Some(position);
            }
            self.move_next();
        }
        None
    }

}



struct Mutator {
    reversed_mutations: BTreeMap<Vec<Molecule>, Molecule>,
}

impl Mutator {
    fn new(mutations: Mutations) -> Self {
        let mut reversed_mutations: BTreeMap<Vec<Molecule>, Molecule> = BTreeMap::new();

        for (source, targets) in &mutations {
            for target in targets {
                match reversed_mutations.entry(target.clone()) {
                    btree_map::Entry::Vacant(entry) => { entry.insert(*source); },
                    btree_map::Entry::Occupied(_) => panic!("Mutation target {:?} is not unique.", target),
                }
            }
        }

        Mutator {
            reversed_mutations: reversed_mutations,
        }
    }

    fn _find_min_steps(&self, source_molecule: &[Molecule], target_molecule: &[Molecule], memoizer: &mut BTreeMap<Vec<Molecule>, Option<u32>>) -> Option<u32> {

        if let Some(result) = memoizer.get(target_molecule) {
            return result.to_owned();
        }

        let result =
            if target_molecule.is_empty() || target_molecule.len() < source_molecule.len() {
                None
            }
            else if target_molecule == source_molecule {
                Some(0)
            }
            else {
                // Look ma, no loop!
                self.reversed_mutations.iter()
                    .flat_map(|(source, target)| SubSlicesIterator::new(target_molecule, source).map(move |index| (source, target, index)))
                    .filter_map(|(source, target, index)| {
                        let mut mutated_molecule = Vec::with_capacity(target_molecule.len() - source.len() + 1);
                        mutated_molecule.extend_from_slice(&target_molecule[..index]);
                        mutated_molecule.push(*target);
                        mutated_molecule.extend_from_slice(&target_molecule[index + source.len()..]);

                        self._find_min_steps(source_molecule, &mutated_molecule, memoizer)
                    })
                    .min()
                    .map(|res| res + 1)
            };

        memoizer.insert(target_molecule.to_owned(), result);
        result
    }

    #[allow(dead_code)]
    fn find_min_steps_rec(&self, source_molecule: &[Molecule], target_molecule: &[Molecule]) -> Option<u32> {
        let mut memoizer = BTreeMap::new();
        self._find_min_steps(source_molecule, target_molecule, &mut memoizer)
    }

    fn find_min_steps_stack(&self, source_molecule: &[Molecule], target_molecule: &[Molecule]) -> Option<u32> {
        let mut stack: VecDeque<(Vec<Molecule>, u32)> = VecDeque::new();
        let mut done: BTreeSet<Vec<Molecule>> = BTreeSet::new();
        stack.push_front((target_molecule.to_owned(), 0));

        let mut result: Option<u32> = None;

        while let Some((molecule, steps)) = stack.pop_front() {
            if molecule.is_empty() || molecule.len() < source_molecule.len() {
                continue;
            }

            if molecule == source_molecule {
                if let Some(ref mut current_min) = result {
                    if *current_min > steps {
                        *current_min = steps;
                    }
                }
                else {
                    result = Some(steps);
                }
                continue;
            }

            // Look ma, no loop!
            stack.extend(
                self.reversed_mutations.iter()
                    .flat_map(|(source, target)| SubSlicesIterator::new(&molecule, source).map(move |index| (source, target, index)))
                    .filter_map(|(source, target, index)| {
                        if *target == 0 && molecule.len() != source.len() {
                            return None;
                        }
                        let mut mutated_molecule = Vec::with_capacity(molecule.len() - source.len() + 1);
                        mutated_molecule.extend_from_slice(&molecule[..index]);
                        mutated_molecule.push(*target);
                        mutated_molecule.extend_from_slice(&molecule[index + source.len()..]);
                        if done.contains(&mutated_molecule) {
                            None
                        } else {
                            done.insert(mutated_molecule.to_owned());
                            Some((mutated_molecule, steps + 1))
                        }
                    })
            );
        }

        result
    }

}

struct Elements {
    hash: BTreeMap<String, u8>
}

impl Elements {

    fn new() -> Self {
        let mut hash = BTreeMap::new();
        hash.insert("e".to_string(), 0);
        Elements { hash: hash }
    }

    fn parse_one(&mut self, s: &str) -> u8 {
        let len = self.hash.len();
        *self.hash.entry(s.to_string()).or_insert(len as u8)
    }

    fn parse(&mut self, s: &str) -> Vec<u8> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"e|[A-Z][a-z]*").unwrap();
        }

        RE.captures_iter(s).map(|cap| {
            self.parse_one(cap.at(0).unwrap())
        }).collect()
    }

    #[allow(dead_code)]
    fn format_one(&self, molecule: &u8) -> &str {
        self.hash.iter().find(|&(_, m)| m == molecule).map(|(s, _)| s).unwrap()
    }

    #[allow(dead_code)]
    fn format(&self, molecule: &[u8]) -> String {
        molecule.iter().map(|m| self.format_one(m)).collect()
    }
}

impl Default for Elements {
    fn default() -> Self {
        Self::new()
    }
}

fn specific_resolution(s: &str) -> u32 {
    Regex::new(r"[A-Z][a-z]*").unwrap().captures_iter(s)
        .fold(-1i32, |total, element| {
            match element.at(0).unwrap() {
                "Rn" | "Ar" => total,
                "Y" => total - 1,
                _ => total + 1,
            }
        }) as u32
}

fn try_specific_solution(s: &str) -> Option<u32> {
    if s.chars().any(|ch| ch == 'Y') {
        Some(specific_resolution(s))
    }
    else {
        None
    }
}

fn main() {

    let stdin = io::stdin();

    let mut lines = stdin.lock().lines().filter_map(|l| l.ok());

    let mut elements = Elements::new();

    let mut mutations: Mutations = BTreeMap::new();

    while let Some(line) = lines.next() {
        if line.is_empty() { break }
        if let Some(index) = line.find(" => ") {
            let (from, to) = line.split_at(index);
            mutations.get_default_mut(elements.parse_one(from))
                .push(elements.parse(&to[4..]));
        }
    }

    let molecule_line = lines.next().unwrap();
    let molecule = elements.parse(&molecule_line);

    let mutated_molecules = mutate(&molecule, &mutations);
    let source_molecule = elements.parse("e");

    let maybe_min_steps = try_specific_solution(&molecule_line)
        .or_else(|| {
            let mutator = Mutator::new(mutations);
            mutator.find_min_steps_stack(&source_molecule, &molecule)
        });

    println!("Mutated molecules count: {}", mutated_molecules.len());

    if let Some(min_steps) = maybe_min_steps {
        println!("Steps needed to find the molecule: {}", min_steps);
    }
    else {
        println!("Can't find the molecule :(");
    }
}
