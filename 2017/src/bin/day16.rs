use std::io::{stdin, Read};

#[derive(Debug)]
enum Action {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

fn apply_actions(actions: &[Action], programs: &mut Vec<char>) {
    for action in actions {
        match *action {
            Action::Exchange(i, j) => programs.swap(i, j),
            Action::Spin(n) => {
                let mut new_programs = Vec::with_capacity(programs.len());
                let index = programs.len() - n;
                new_programs.extend_from_slice(&programs[index..]);
                new_programs.extend_from_slice(&programs[0..index]);
                *programs = new_programs
            }
            Action::Partner(a, b) => {
                let pa = programs.iter().position(|p| p == &a).unwrap();
                let pb = programs.iter().position(|p| p == &b).unwrap();
                programs.swap(pa, pb);
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).expect(
        "Failed to read stdin",
    );

    let actions: Vec<_> = input
        .trim()
        .split(',')
        .map(|desc| match desc.chars().nth(0).unwrap() {
            'x' => {
                let pos = desc.chars().position(|ch| ch == '/').unwrap();
                Action::Exchange(
                    desc[1..pos].parse().unwrap(),
                    desc[pos + 1..].parse().unwrap(),
                )
            }
            'p' => Action::Partner(desc.chars().nth(1).unwrap(), desc.chars().nth(3).unwrap()),
            's' => Action::Spin(desc[1..].parse().unwrap()),
            _ => unreachable!(),
        })
        .collect();

    let programs: Vec<_> = (0..16).map(|i| (b'a' + i) as char).collect();
    {
        let mut first_round = programs.clone();
        apply_actions(&actions, &mut first_round);
        println!("Part 1: {}", first_round.iter().collect::<String>());
    }

    {
        let mut billion_rounds = programs.clone();
        let mut period = 0;
        let first = programs.clone();

        for _ in 0..1_000_000_000 {
            apply_actions(&actions, &mut billion_rounds);
            period += 1;
            if billion_rounds == first {
                break;
            }
        }

        // Take a shortcut!
        for _ in 0..(1_000_000_000 % period) {
            apply_actions(&actions, &mut billion_rounds);
        }

        println!("Part 2: {}", billion_rounds.iter().collect::<String>());
    }
}
