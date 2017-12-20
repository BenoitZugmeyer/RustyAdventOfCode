extern crate regex;
use std::io::{stdin, BufRead};
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
struct Coords {
    x: i32,
    y: i32,
    z: i32,
}

impl std::ops::Add<Coords> for Coords {
    type Output = Coords;

    fn add(self, other: Coords) -> Coords {
        Coords {
            x: self.x.checked_add(other.x).unwrap(),
            y: self.y.checked_add(other.y).unwrap(),
            z: self.z.checked_add(other.z).unwrap(),
        }
    }
}


#[derive(Debug, Clone)]
struct Particle {
    position: Coords,
    velocity: Coords,
    acceleration: Coords,
}

impl Particle {
    fn steps(&self, n: usize) -> Particle {
        let mut result = self.clone();

        for _ in 0..n {
            result.velocity = result.velocity + result.acceleration;
            result.position = result.position + result.velocity;
        }

        result
    }
}

fn filter_collisions(particles: &mut Vec<Particle>) {
    let mut positions: HashSet<Coords> = HashSet::new();
    let mut colided_positions: HashSet<Coords> = HashSet::new();
    for particle in particles.iter() {
        if positions.contains(&particle.position) {
            colided_positions.insert(particle.position.clone());
        } else {
            positions.insert(particle.position.clone());
        }
    }

    if !colided_positions.is_empty() {
        *particles = particles
            .iter()
            .cloned()
            .filter(|p| !colided_positions.contains(&p.position))
            .collect();
    }
}

fn main() {
    let stdin = stdin();

    let re = Regex::new(r"-?\d+").expect("failed to parse regex");

    let particles: Vec<_> = stdin
        .lock()
        .lines()
        .filter_map(|l| l.ok())
        .map(|line| {
            let values = re.captures_iter(&line)
                .map(|capture| capture.get(0).unwrap().as_str().parse().unwrap())
                .collect::<Vec<i32>>();
            Particle {
                position: Coords {
                    x: values[0],
                    y: values[1],
                    z: values[2],
                },
                velocity: Coords {
                    x: values[3],
                    y: values[4],
                    z: values[5],
                },
                acceleration: Coords {
                    x: values[6],
                    y: values[7],
                    z: values[8],
                },
            }
        })
        .collect();


    if let Some((index, _)) = particles.iter().enumerate().min_by_key(|&(_, particle)| {
        let long_term_particle = particle.steps(5000);
        long_term_particle
            .position
            .x
            .abs()
            .checked_add(long_term_particle.position.y.abs())
            .unwrap()
            .checked_add(long_term_particle.position.z.abs())
            .unwrap()
    })
    {
        println!("Part 1: {}", index);
    }

    {
        let mut particles = particles.clone();
        for _ in 0..100 {
            filter_collisions(&mut particles);
            particles = particles.iter().map(|p| p.steps(1)).collect();
        }
        println!("Part 2: {}", particles.len());
    }


}
