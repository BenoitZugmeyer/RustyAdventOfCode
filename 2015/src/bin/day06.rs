#[macro_use] extern crate lazy_static;
extern crate regex;

use std::io;
use std::io::BufRead;
use regex::Regex;

#[derive(Debug)]
enum ActionType {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug)]
struct Action {
    todo: ActionType,
    ax: usize,
    ay: usize,
    bx: usize,
    by: usize,
}

impl Action {

    fn apply_on_grid<T, F: Fn(&mut T)>(&self, lights: &mut [T], f: F) {
        for x in self.ax ..= self.bx {
            for y in self.ay ..= self.by {
                let light = &mut lights[x * 1000 + y];
                f(light);
            }
        }
    }

    fn apply_binary(&self, lights: &mut [bool]) {
        self.apply_on_grid(lights, |light| {
            match self.todo {
                ActionType::TurnOn => *light = true,
                ActionType::TurnOff => *light = false,
                ActionType::Toggle => *light = !*light,
            }
        });
    }

    fn apply_multilevel(&self, lights: &mut [u8]) {
        self.apply_on_grid(lights, |light| {
            match self.todo {
                ActionType::TurnOn => *light += 1,
                ActionType::TurnOff => if *light > 0 { *light -= 1 },
                ActionType::Toggle => *light += 2,
            }
        });
    }
}

fn parse(s: &str) -> Option<Action> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(toggle|turn on|turn off) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    }
    RE.captures(s).map(|ref m| {
        Action {
            todo: match m.at(1).unwrap() {
                "turn on" => ActionType::TurnOn,
                "turn off" => ActionType::TurnOff,
                _ => ActionType::Toggle,
            },
            ax: m.at(2).unwrap().parse().unwrap(),
            ay: m.at(3).unwrap().parse().unwrap(),
            bx: m.at(4).unwrap().parse().unwrap(),
            by: m.at(5).unwrap().parse().unwrap(),
        }
    })
}

fn main() {

    let stdin = io::stdin();

    let mut binary_lights = [false; 1_000_000];
    let mut multilevel_lights = [0u8; 1_000_000];

    let actions = stdin.lock().lines()
        .filter_map(|l| l.ok())
        .filter_map(|ref line| parse(line));

    for action in actions {
        action.apply_binary(&mut binary_lights);
        action.apply_multilevel(&mut multilevel_lights);
    }

    println!("Binary lights lit: {}", binary_lights.iter().filter(|&&on| on).count());
    println!("Multi level lights brightness: {}", multilevel_lights.iter().fold(0u32, |t, &l| t + u32::from(l)));
}

