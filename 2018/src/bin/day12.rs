use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::io::{stdin, BufRead};

fn parse_pots(iter: &mut impl Iterator<Item = char>) -> Vec<bool> {
    iter.take_while(|&ch| ch == '.' || ch == '#')
        .map(|ch| ch == '#')
        .collect()
}

fn main() {
    let stdin = stdin();
    let mut lines = stdin.lock().lines().filter_map(|l| l.ok());
    let first_line = lines.next().expect("Failed to read input line");

    // Parse initial state into a boolean vector
    let initial_state = parse_pots(&mut first_line.chars().skip(15));

    // Parse rules into a HashMap
    let rules: HashMap<(bool, bool, bool, bool, bool), bool> = lines
        .skip(1)
        .filter_map(|line| {
            let mut chars = line.chars();
            let bools = parse_pots(&mut chars);
            if bools.len() == 5 {
                Some((
                    (bools[0], bools[1], bools[2], bools[3], bools[4]),
                    chars.nth(3)? == '#',
                ))
            } else {
                None
            }
        })
        .collect();

    // The size to add when there is not enough space before the first filled plant or after the
    // last one
    let padding_size = 20;

    // Start with the initial state
    let mut state = initial_state;
    // No padding yet
    let mut padding_start = 0;
    // Collect previous sums for Part 2
    let mut previous_sums = VecDeque::new();

    for generation in 0.. {
        // Compute the sum for this state
        let sum = state
            .iter()
            .enumerate()
            .map(|(index, &b)| {
                if b {
                    index as i64 - padding_start as i64
                } else {
                    0
                }
            })
            .sum::<i64>();

        if generation == 20 {
            // Part 1: display the sum at generation 20
            println!("Part 1: {}", sum);
        }

        // Callect the 5 previous sums
        previous_sums.push_back(sum);
        if previous_sums.len() > 5 {
            // If all previous sums difference are equal, we expect that all further generation sum
            // will increase by the same amount
            let factor = previous_sums[1] - previous_sums[0];
            if previous_sums
                .iter()
                .tuple_windows()
                .map(|(a, b)| b - a)
                .all(|f| f == factor)
            {
                // Compute the sum for the step 50_000_000_000
                println!("Part 2: {}", sum + ((50_000_000_000 - generation) * factor));
                break;
            } else {
                // Drop the oldest sum, so we keep the previous_sums length to 5
                previous_sums.pop_front();
            }
        }

        // Compute the next generation

        // First, we pad the state with empty pots, at the beginning and the end, if there is less
        // than 3 empty pots on each side.
        let mut tmp_state = Vec::new();

        if state.iter().position(|&b| b).unwrap() < 3 {
            tmp_state.extend(std::iter::repeat(false).take(padding_size));
            padding_start += padding_size;
        }

        tmp_state.extend(&state);

        if state.len() - state.iter().rposition(|&b| b).unwrap() - 1 < 3 {
            tmp_state.extend(std::iter::repeat(false).take(padding_size));
        }

        // Then we can iterate over each pots to compute the future state. Start with two empty
        // pots, because our window algorith is 'eating' the first two.
        state = vec![false; 2];
        state.extend(tmp_state.windows(5).map(|bools| {
            rules
                .get(&(bools[0], bools[1], bools[2], bools[3], bools[4]))
                .cloned()
                .unwrap_or(false)
        }));
    }
}
