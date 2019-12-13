use crate::intcode::Value;
use itertools::Itertools;
use std::convert::TryFrom;

fn as_usize(v: Value) -> usize {
    usize::try_from(v).unwrap()
}
struct Screen {
    tiles: Vec<Vec<Value>>,
    score: Value,
    paddle_x: usize,
    ball_x: usize,
}

impl Screen {
    #[allow(dead_code)]
    fn new(initial_instructions: &[Value]) -> Self {
        let mut max_x = 0;
        let mut max_y = 0;
        for (x, y, _id) in initial_instructions.iter().cloned().tuples() {
            if x >= 0 {
                max_x = max_x.max(x);
                max_y = max_y.max(y);
            }
        }
        let tiles = vec![vec![0; as_usize(max_x) + 1]; as_usize(max_y) + 1];
        let mut screen = Self {
            tiles,
            score: 0,
            paddle_x: 0,
            ball_x: 0,
        };
        screen.update(initial_instructions);
        screen
    }

    fn update(&mut self, instructions: &[Value]) {
        for (x, y, id) in instructions.iter().cloned().tuples() {
            if x == -1 {
                self.score = id;
            } else {
                let (x, y) = (as_usize(x), as_usize(y));
                self.tiles[y][x] = id;
                if id == 3 {
                    self.paddle_x = x;
                } else if id == 4 {
                    self.ball_x = x;
                }
            }
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        for row in &self.tiles {
            for tile in row {
                print!(
                    "{}",
                    match tile {
                        0 => ' ',
                        1 => '\u{2588}',
                        2 => '\u{25FB}',
                        3 => '_',
                        4 => 'o',
                        _ => panic!("Unknown id {}", tile),
                    }
                );
            }
            println!();
        }
        println!("{}", self.score);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    use crate::intcode::{Program, ProgramResult};
    fn get_program() -> Program {
        util::input(13).next().expect("No input").parse().unwrap()
    }

    #[test]
    fn part_1() {
        let mut program = get_program();
        let instructions = program.run(&[]).unwrap();
        assert_eq!(
            Some(
                instructions
                    .iter()
                    .tuples()
                    .filter(|(_x, _y, id)| **id == 2)
                    .count()
            ),
            util::answer(13, 1)
        );
    }

    #[test]
    fn part_2() {
        let mut program = get_program();
        program.write_memory(0, 2);

        let mut screen = match program.run(&[]) {
            ProgramResult::NeedInput(instructions) => Screen::new(&instructions),
            _ => panic!("Nope"),
        };

        loop {
            let joystick = if screen.paddle_x < screen.ball_x {
                1
            } else if screen.paddle_x > screen.ball_x {
                -1
            } else {
                0
            };

            match program.run(&[joystick]) {
                ProgramResult::NeedInput(instructions) => screen.update(&instructions),
                ProgramResult::Halt(instructions) => {
                    screen.update(&instructions);
                    break;
                }
            }
        }

        assert_eq!(Some(screen.score), util::answer(13, 2));
    }
}
