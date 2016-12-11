#[macro_use]
extern crate nom;

use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use std::collections::{HashSet, BTreeSet, VecDeque, BTreeMap};
use std::io::stdin;
use std::io::Read;

#[derive(Debug, Ord, Eq, PartialOrd, PartialEq, Clone, Hash)]
enum Object {
    Generator(u64),
    Microchip(u64),
}

impl Object {
    fn is_generator(&self) -> bool {
        match *self {
            Object::Generator(_) => true,
            Object::Microchip(_) => false,
        }
    }

    fn name(&self) -> u64 {
        match *self {
            Object::Generator(name) |
            Object::Microchip(name) => name,
        }
    }
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match *self {
            Object::Generator(name) => write!(f, "G({})", name)?,
            Object::Microchip(name) => write!(f, "M({})", name)?,
        }
        Ok(())
    }
}

fn new_string(bits: &[u8]) -> String {
    String::from_utf8_lossy(bits).into_owned()
}

fn hash(bytes: &[u8]) -> u64 {
    let mut h = DefaultHasher::new();
    bytes.hash(&mut h);
    h.finish()
}

named!(
    parse_instruction(&[u8]) -> Vec<BTreeSet<Object>>,
    many0!(
        do_parse!(
            tag!("The ") >>
            take_while1_s!(nom::is_alphabetic) >>
            tag!(" floor contains ") >>
            res: alt!(
                many1!(
                    do_parse!(
                        tag!("a ") >>
                        name: take_while1_s!(nom::is_alphabetic) >>
                        res: alt!(
                            map!(tag!("-compatible microchip"), |_| {
                                Object::Microchip(hash(name))
                            })
                            |
                            map!(tag!(" generator"), |_| {
                                Object::Generator(hash(name))
                            })
                        ) >>
                        opt!(tag!(",")) >>
                        opt!(tag!(" ")) >>
                        opt!(tag!("and ")) >>
                        (res)
                    )
                )
                |
                map!(tag!("nothing relevant"), |_| vec![])
            )>>
            tag!(".") >>
            opt!(tag!("\n")) >>
            (res.into_iter().collect())
        )
    )
);

fn is_valid(floor_objects: &BTreeSet<Object>) -> bool {
    let has_generator = floor_objects.iter().any(|o| o.is_generator());
    floor_objects.iter().all(|object| {
        object.is_generator() ||
            floor_objects.contains(&Object::Generator(object.name())) || // is connected
            !has_generator
    })
}

#[derive(Debug, Ord, Eq, PartialOrd, PartialEq, Clone, Hash)]
struct State {
    floor: usize,
    objects: Vec<BTreeSet<Object>>,
}

impl State {
    #[allow(dead_code)]
    fn print(&self) {
        for i in (0..4).rev() {
            print!("{}F {} ", i + 1, if self.floor == i { "E" } else { "." });
            for o in &self.objects[i] {
                print!("{} ", o);
            }
            println!("");
        }
    }

    fn step(&self) -> Vec<State> {
        let mut res = Vec::new();

        if self.floor > 0 {
            self.gen_states(self.floor - 1, &mut res);
        }

        if self.floor < 3 {
            self.gen_states(self.floor + 1, &mut res);
        }

        res
    }

    fn gen_states(&self, next_floor: usize, mut res: &mut Vec<State>) {
        for a in &self.objects[self.floor] {
            if next_floor > self.floor {
                if !self.gen_states_couples(a, next_floor, &mut res) {
                    self.gen_states_single(a, next_floor, &mut res);
                }
            } else if !self.gen_states_single(a, next_floor, &mut res) {
                self.gen_states_couples(a, next_floor, &mut res);
            }
        }
    }

    fn gen_states_couples(&self, a: &Object, next_floor: usize, mut res: &mut Vec<State>) -> bool {
        let mut result = false;
        for b in &self.objects[self.floor] {
            if a > b {
                if let Some(s) = self.gen_next_state(&[a, b], next_floor) {
                    res.push(s);
                    result = true;
                }
            }
        }
        result
    }

    fn gen_states_single(&self, a: &Object, next_floor: usize, res: &mut Vec<State>) -> bool {
        if let Some(s) = self.gen_next_state(&[a], next_floor) {
            res.push(s);
            true
        } else {
            false
        }
    }

    fn gen_next_state(&self, n: &[&Object], next_floor: usize) -> Option<State> {
        let mut new_objects = Vec::new();
        for (i, floor_objects) in self.objects.iter().enumerate() {
            let mut clone = floor_objects.clone();
            if i == next_floor {
                for o in n {
                    clone.insert((*o).clone());
                }
                if !is_valid(&clone) {
                    return None;
                }
            } else if i == self.floor {
                for o in n {
                    clone.remove(o);
                }
                if !is_valid(&clone) {
                    return None;
                }
            }
            new_objects.push(clone);
        }
        Some(State {
            floor: next_floor,
            objects: new_objects,
        })
    }

    fn is_final(&self) -> bool {
        self.objects.iter().take(3).all(|o| o.is_empty())
    }

    fn hash(&self) -> (usize, Vec<Vec<(usize, bool)>>) {
        let mut set = BTreeMap::new();

        (self.floor,
         self.objects
             .iter()
             .map(|row| {
                let mut a: Vec<_> = row.iter()
                    .map(|o| {
                        let len = set.len();
                        (*set.entry(o.name()).or_insert(len), o.is_generator())
                    })
                    .collect();
                a.sort();
                a
            })
             .collect())
    }
}

fn get_min_steps(objects: &[BTreeSet<Object>]) -> Option<u32> {
    let state = State {
        floor: 0,
        objects: objects.to_vec(),
    };
    let mut previous_states = HashSet::new();
    let mut states = VecDeque::new();
    states.push_back((state, 0));
    let mut steps = Vec::new();

    while let Some((state, step)) = states.pop_front() {

        if state.is_final() {
            steps.push(step);
        }

        for state in state.step() {
            let hash = state.hash();
            if !previous_states.contains(&hash) {
                previous_states.insert(hash);
                states.push_back((state, step + 1));
            }
        }
    }

    steps.into_iter().min()
}

fn main() {

    let mut input = Vec::new();
    stdin().read_to_end(&mut input).expect("Failed to read stdin");
    let (rest, mut objects) = parse_instruction(&input).unwrap();
    if !rest.is_empty() {
        panic!("Can't parse the rest of the input: {}", new_string(rest));
    }

    println!("Part 1: {:?}", get_min_steps(&objects));

    objects[0].insert(Object::Generator(hash(b"elerium")));
    objects[0].insert(Object::Microchip(hash(b"elerium")));
    objects[0].insert(Object::Generator(hash(b"dilithium")));
    objects[0].insert(Object::Microchip(hash(b"dilithium")));
    println!("Part 2: {:?}", get_min_steps(&objects));
}
