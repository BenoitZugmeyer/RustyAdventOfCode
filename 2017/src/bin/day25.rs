use std::io::{stdin, BufRead};

#[derive(Debug)]
struct Action {
    write: bool,
    move_: isize,
    next_state: usize,
}

type State = (Action, Action);

#[inline]
fn state_from_char(ch: char) -> usize {
    ch as usize - 'A' as usize
}

fn parse_head(lines: &[String]) -> (usize, u32) {
    (
        lines[0]
            .chars()
            .nth(15)
            .map(state_from_char)
            .expect("parse head initial state"),
        lines[1][36..lines[1].len() - 7]
            .parse()
            .expect("parse head steps"),
    )
}

fn parse_action(lines: &[String]) -> Action {
    Action {
        write: lines[0].chars().nth(22) == Some('1'),
        move_: if lines[1].ends_with("right.") { 1 } else { -1 },
        next_state: lines[2]
            .chars()
            .nth(26)
            .map(state_from_char)
            .expect("parse action next"),
    }
}

fn parse_state(lines: &[String]) -> State {
    (parse_action(&lines[2..5]), parse_action(&lines[6..9]))
}

struct Memory {
    negative: Vec<bool>,
    positive: Vec<bool>,
}

impl Memory {
    fn new() -> Self {
        Memory {
            negative: Vec::new(),
            positive: Vec::new(),
        }
    }

    fn get(&mut self, position: isize) -> &mut bool {
        if position < 0 {
            get_memory(&mut self.negative, (-position - 1) as usize)
        } else {
            get_memory(&mut self.positive, position as usize)
        }
    }
}

#[inline]
fn get_memory(memory: &mut Vec<bool>, index: usize) -> &mut bool {
    if memory.len() == index {
        memory.push(false);
    }
    &mut memory[index]
}

fn main() {
    let stdin = stdin();
    let lines: Vec<_> = stdin.lock().lines().filter_map(|l| l.ok()).collect();
    let (mut state_index, steps) = parse_head(&lines[0..2]);
    let states: Vec<_> = (3..lines.len())
        .step_by(10)
        .map(|i| parse_state(&lines[i..i + 9]))
        .collect();
    let mut memory = Memory::new();
    let mut position: isize = 0;
    for _ in 0..steps {
        let entry = memory.get(position);
        let action = if !*entry {
            &states[state_index].0
        } else {
            &states[state_index].1
        };
        *entry = action.write;
        position += action.move_;
        state_index = action.next_state;
    }
    println!(
        "Part 1: {}",
        memory.positive.into_iter().filter(|&b| b).count()
            + memory.negative.into_iter().filter(|&b| b).count()
    );
}
