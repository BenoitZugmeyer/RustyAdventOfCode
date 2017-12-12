extern crate regex;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::{stdin, BufRead};

type Program = u32;

fn populate_group(
    programs: &HashMap<Program, Vec<Program>>,
    mut group: &mut HashSet<Program>,
    program: Program,
) {
    if group.contains(&program) {
        return;
    }

    group.insert(program);
    for other in &programs[&program] {
        populate_group(programs, &mut group, *other);
    }
}

fn main() {
    let stdin = stdin();

    let re = Regex::new(r"\d+").expect("failed to parse regex");

    let programs = stdin
        .lock()
        .lines()
        .filter_map(|l| l.ok())
        .map(|line| {
            let values = re.captures_iter(&line)
                .map(|capture| capture.get(0).unwrap().as_str().parse().unwrap())
                .collect::<Vec<Program>>();
            (values[0], values[1..].to_vec())
        })
        .collect::<HashMap<Program, Vec<Program>>>();

    {
        let mut group = HashSet::new();
        populate_group(&programs, &mut group, 0);
        println!("Part 1: {}", group.len());
    }

    {
        let mut in_group = HashSet::new();
        let mut count = 0;
        for program in programs.keys() {
            if !in_group.contains(program) {
                count += 1;
                populate_group(&programs, &mut in_group, *program);
            }
        }
        println!("Part 2: {}", count);
    }
}
