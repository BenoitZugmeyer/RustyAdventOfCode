use crate::intcode::Value;
use crate::point::{Direction, Point};
use std::collections::HashSet;
use std::convert::TryFrom;

#[allow(dead_code)]
fn get_state(map: &[u8]) -> (HashSet<Point>, (Point, Direction)) {
    let mut scaffolds = HashSet::new();
    let mut location = Point::new(0, 0);
    let mut vacum_robot = None;
    for ch in map {
        match ch {
            35 => {
                scaffolds.insert(location);
                location.x += 1;
            }
            46 => {
                location.x += 1;
            }
            94 => {
                vacum_robot = Some((location, Direction::North));
                location.x += 1;
            }
            10 => {
                location.x = 0;
                location.y += 1;
            }
            ch => {
                panic!("Unhandled char {}", ch);
            }
        }
    }

    (scaffolds, vacum_robot.unwrap())
}

fn actions_len(actions: &[char]) -> u32 {
    actions
        .iter()
        .map(|ch| {
            match *ch {
                'L' | 'R' | 'A' | 'B' | 'C' => 2,
                // one digit
                ch if ch <= '9' => 2,
                // two digits
                _ => 3,
            }
        })
        .sum::<u32>()
        - 1
}

#[allow(dead_code)]
fn format_actions(actions: &[char]) -> Vec<Value> {
    let mut output = Vec::new();
    for ch in actions {
        match *ch {
            'L' | 'R' | 'A' | 'B' | 'C' => output.push(Value::from(*ch as u8)),
            // one digit
            ch if ch <= '9' => output.push(Value::from(ch as u8)),
            // two digits
            ch => {
                let v = ch as u8 - b'0';
                output.push(Value::from((v / 10) + b'0'));
                output.push(Value::from((v % 10) + b'0'));
            }
        }
        output.push(Value::from(b','));
    }
    *output.last_mut().unwrap() = Value::from(b'\n');
    output
}

fn find_routines_rec(
    actions: &[char],
    previous_routines: &[Vec<char>],
    main_routine: &[char],
) -> Option<(Vec<char>, Vec<Vec<char>>)> {
    if actions.is_empty() {
        return Some((main_routine.to_vec(), previous_routines.to_owned()));
    }

    for l in 1..=actions.len() {
        let new_routine = &actions[0..l];
        if actions_len(new_routine) > 20 {
            break;
        }
        let (previous_routines, p) = if let Some(p) = previous_routines
            .iter()
            .position(|r| r.as_slice() == new_routine)
        {
            (previous_routines.to_owned(), p)
        } else if previous_routines.len() < 3 {
            let mut previous_routines = previous_routines.to_owned();
            previous_routines.push(new_routine.to_vec());
            let p = previous_routines.len() - 1;
            (previous_routines, p)
        } else {
            continue;
        };

        let new_main_routine: Vec<_> = main_routine
            .iter()
            .cloned()
            .chain(std::iter::once((b'A' + u8::try_from(p).unwrap()) as char))
            .collect();
        if actions_len(&new_main_routine) > 20 {
            break;
        }
        let result = find_routines_rec(&actions[l..], &previous_routines, &new_main_routine);
        if result.is_some() {
            return result;
        }
    }

    None
}

#[allow(dead_code)]
fn find_routines(
    scaffolds: &HashSet<Point>,
    vacum_robot_position: Point,
    vacum_robot_direction: Direction,
) -> Option<(Vec<char>, Vec<Vec<char>>)> {
    let actions = get_actions(&scaffolds, vacum_robot_position, vacum_robot_direction);

    let previous_routines = Vec::new();
    find_routines_rec(&actions, &previous_routines, &[])
}

fn get_actions(
    scaffolds: &HashSet<Point>,
    mut vacum_robot_position: Point,
    mut vacum_robot_direction: Direction,
) -> Vec<char> {
    let mut actions = vec![];

    loop {
        let mut n = 0;
        loop {
            let next_position = vacum_robot_position.walk(vacum_robot_direction);
            if !scaffolds.contains(&next_position) {
                break;
            }
            vacum_robot_position = next_position;
            n += 1;
        }

        if n > 0 {
            actions.push((n + b'0') as char);
        }

        let command = if scaffolds
            .contains(&vacum_robot_position.walk(vacum_robot_direction.left()))
        {
            vacum_robot_direction = vacum_robot_direction.left();
            'L'
        } else if scaffolds.contains(&vacum_robot_position.walk(vacum_robot_direction.right())) {
            vacum_robot_direction = vacum_robot_direction.right();
            'R'
        } else {
            break;
        };

        actions.push(command);
    }

    actions
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intcode::Program;
    use crate::util;
    use std::convert::TryFrom;

    fn get_program() -> Program {
        util::input(17).next().expect("No input").parse().unwrap()
    }

    fn map_from_output(output: &[Value]) -> Vec<u8> {
        output.iter().map(|n| u8::try_from(*n).unwrap()).collect()
    }

    #[test]
    fn part_1() {
        let (scaffolds, _) = get_state(&map_from_output(&get_program().run(&[]).unwrap()));
        assert_eq!(
            Some(
                scaffolds
                    .iter()
                    .filter_map(|scaffold| {
                        if scaffold.iter_nearby().all(|p| scaffolds.contains(&p)) {
                            Some(scaffold.x * scaffold.y)
                        } else {
                            None
                        }
                    })
                    .sum::<i32>()
            ),
            util::answer(17, 1)
        );
    }

    #[test]
    fn actions_len_test() {
        assert_eq!(actions_len(&['A']), 1);
        assert_eq!(actions_len(&['A', 'C']), 3);
        assert_eq!(actions_len(&['<']), 2);
        assert_eq!(actions_len(&['4']), 1);
        assert_eq!(actions_len(&['<', '4']), 4);
    }

    #[test]
    fn format_actions_test() {
        assert_eq!(
            format_actions(&['A']),
            vec![Value::from(b'A'), Value::from(b'\n')]
        );
        assert_eq!(
            format_actions(&['A', 'C']),
            vec![
                Value::from(b'A'),
                Value::from(b','),
                Value::from(b'C'),
                Value::from(b'\n')
            ]
        );
        assert_eq!(
            format_actions(&['<']),
            vec![Value::from(b'1'), Value::from(b'2'), Value::from(b'\n')]
        );
        assert_eq!(
            format_actions(&['4']),
            vec![Value::from(b'4'), Value::from(b'\n')]
        );
        assert_eq!(
            format_actions(&['<', '4']),
            vec![
                Value::from(b'1'),
                Value::from(b'2'),
                Value::from(b','),
                Value::from(b'4'),
                Value::from(b'\n')
            ]
        );
    }

    #[test]
    fn part_2() {
        let mut program = get_program();

        let (scaffolds, (vacum_robot_position, vacum_robot_direction)) =
            get_state(&map_from_output(&program.clone().run(&[]).unwrap()));

        let (main_routine, routines) =
            find_routines(&scaffolds, vacum_robot_position, vacum_robot_direction).unwrap();

        program.write_memory(0, 2);

        program.run(&format_actions(&main_routine));
        for routine in routines {
            program.run(&format_actions(&routine));
        }

        let output = program
            .run(&[Value::from(b'n'), Value::from(b'\n')])
            .get_output();
        assert_eq!(output.last().cloned(), util::answer(17, 2));
    }
}
