use std::collections::{HashMap, HashSet};
use std::io::{stdin, BufRead};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coordinates {
    x: isize,
    y: isize,
}

impl Coordinates {
    fn next(&self, direction: char) -> Self {
        match direction {
            'N' => Self {
                x: self.x,
                y: self.y - 1,
            },
            'E' => Self {
                x: self.x + 1,
                y: self.y,
            },
            'W' => Self {
                x: self.x - 1,
                y: self.y,
            },
            'S' => Self {
                x: self.x,
                y: self.y + 1,
            },
            _ => panic!("Invalid direction {}", direction),
        }
    }
}

fn build_map(input: &str) -> HashMap<Coordinates, usize> {
    let root = Coordinates { x: 0, y: 0 };

    // The map associates a coordinates with the distance to reach it.  We start at room 0,0, the
    // distance is 0
    let mut map = HashMap::new();
    map.insert(root, 0);

    // A state is a couple of two sets of coordinates.  We add a state every time we encouter a
    // branch (the '(' character).
    // * The first set represents the coordinates that can be reached at the current position in
    // the regex.
    // * The second set are coordinates that was reachable at the start of the regex branch.
    let mut states = vec![(
        {
            let mut start = HashSet::new();
            start.insert(root);
            start
        },
        HashSet::new(),
    )];

    for ch in input.chars() {
        match ch {
            'W' | 'E' | 'S' | 'N' => {
                // Mutate the reachable positions to the 'next' reachable positions in the given
                // direction.
                let state = states.last_mut().unwrap();
                state.0 = state
                    .0
                    .iter()
                    .map(|branch| {
                        let next = branch.next(ch);
                        // Store the distance to access the future room, which is the distance to
                        // access the current room + 1 (use the minimal distance if there is
                        // already one)
                        let distance = map[&branch] + 1;
                        map.entry(next)
                            .and_modify(|d| *d = std::cmp::min(*d, distance))
                            .or_insert(distance);
                        next
                    })
                    .collect();
            }
            '(' => {
                // Branching start.

                // "Replace" the last state reachable coordinates with an empty set (those will be
                // added afterward, on each branch end).
                let state = states.pop().unwrap();
                states.push((HashSet::new(), state.1));

                // Add a new state by storing the reachable coordinates.
                states.push((state.0.clone(), state.0));
            }
            '|' => {
                // Single branch end.

                // Add reachable coordinates to the last state
                let state = states.pop().unwrap();
                states.last_mut().unwrap().0.extend(&state.0);

                // Add a new state by storing the reachable coordinates at the beginning of the
                // branching.
                states.push((state.1.clone(), state.1));
            }
            ')' => {
                // Branching end.
                let state = states.pop().unwrap();
                // Add reachable coordinates to the last state
                states.last_mut().unwrap().0.extend(&state.0);
            }
            '^' => {}
            '$' => break,
            _ => panic!("Invalid char {}", ch),
        }
    }

    map
}

fn main() {
    let line = stdin()
        .lock()
        .lines()
        .filter_map(|l| l.ok())
        .next()
        .unwrap();

    let map = build_map(&line);
    println!("Part 1: {}", *map.values().max().unwrap());
    println!(
        "Part 2: {}",
        map.values().filter(|&distance| *distance >= 1000).count()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_part_1(input: &str) -> usize {
        *build_map(input).values().max().unwrap()
    }

    #[test]
    fn test_part_1() {
        assert_eq!(run_part_1("^WNE$"), 3);
        assert_eq!(run_part_1("^ENWWW(NEEE|SSE(EE|N))$"), 10);
        assert_eq!(run_part_1("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$"), 18);
        assert_eq!(
            run_part_1("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$"),
            23
        );
        assert_eq!(
            run_part_1("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$"),
            31
        );
    }
}
