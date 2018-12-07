use std::collections::{BTreeMap, BTreeSet};
use std::io::{stdin, BufRead};

fn main() {
    let mut remaining_steps = BTreeMap::new();

    for (a, b) in stdin()
        .lock()
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| Some((l.chars().nth(5)?, l.chars().nth(36)?)))
    {
        remaining_steps
            .entry(b)
            .or_insert_with(BTreeSet::new)
            .insert(a);
        remaining_steps.entry(a).or_insert_with(BTreeSet::new);
    }

    {
        let mut remaining_steps = remaining_steps.clone();
        let mut steps_done = BTreeSet::new();
        let mut steps_order = String::new();
        while let Some((&next_step, _)) = remaining_steps
            .iter()
            .find(|(_, requirements)| steps_done.is_superset(requirements))
        {
            steps_done.insert(next_step);
            remaining_steps.remove(&next_step);
            steps_order.push(next_step);
        }

        println!("Part 1: {}", steps_order);
    }

    {
        let mut remaining_steps = remaining_steps.clone();

        let max_workers = 5;
        let mut working_workers = BTreeSet::new();
        let mut time = 0;

        loop {
            // Collect the next steps to do
            let next_steps: Vec<char> = remaining_steps
                .iter()
                // Filter out steps with unfinished requirements
                .filter(|(_, requirements)| requirements.is_empty())
                // Take at most N steps where N is the number of free workers
                .take(max_workers - working_workers.len())
                .map(|(s, _)| *s)
                .collect();

            for &next_step in &next_steps {
                let duration = next_step as u16 - 'A' as u16 + 61;
                remaining_steps.remove(&next_step);
                working_workers.insert((time + duration, next_step));
            }

            if let Some((end, step)) = working_workers.iter().cloned().next() {
                // The work is finished, consider the step done
                working_workers.remove(&(end, step));
                for requirements in remaining_steps.values_mut() {
                    requirements.remove(&step);
                }
                time = end
            } else {
                println!("Part 2: {}", time);
                break;
            }
        }
    }
}
