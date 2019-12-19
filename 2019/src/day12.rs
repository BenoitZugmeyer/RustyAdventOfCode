use crate::util;
use itertools::Itertools;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point3D {
    dimensions: [i64; 3],
}

impl Point3D {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self {
            dimensions: [x, y, z],
        }
    }
}

impl std::str::FromStr for Point3D {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars().enumerate();
        let is_part_of_number = |ch: char| ch == '-' || ch >= '0' && ch <= '9';
        let mut next_number = || {
            let (start, _) = chars
                .by_ref()
                .find(|(_, ch)| is_part_of_number(*ch))
                .unwrap();
            let (end, _) = chars
                .by_ref()
                .find(|(_, ch)| !is_part_of_number(*ch))
                .unwrap();
            s[start..end].parse::<i64>().unwrap()
        };

        Ok(Self::new(next_number(), next_number(), next_number()))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Moon {
    position: Point3D,
    velocity: Point3D,
}

impl Moon {
    #[allow(dead_code)]
    fn new(position: Point3D) -> Self {
        Self {
            position,
            velocity: Point3D::new(0, 0, 0),
        }
    }
}

#[allow(dead_code)]
fn step(moons: &[Moon]) -> Vec<Moon> {
    let mut new_moons = moons.to_vec();

    for ((index_a, moon_a), (index_b, moon_b)) in moons.iter().enumerate().tuple_combinations() {
        for d in 0..3 {
            let a = moon_a.position.dimensions[d];
            let b = moon_b.position.dimensions[d];
            let diff = match a.cmp(&b) {
                Ordering::Greater => 1,
                Ordering::Less => -1,
                _ => 0,
            };
            new_moons[index_a].velocity.dimensions[d] -= diff;
            new_moons[index_b].velocity.dimensions[d] += diff;
        }
    }

    for moon in &mut new_moons {
        for d in 0..3 {
            moon.position.dimensions[d] += moon.velocity.dimensions[d];
        }
    }

    new_moons
}

#[allow(dead_code)]
fn compute_energy(moons: &[Moon]) -> i64 {
    moons
        .iter()
        .map(|moon| {
            let pos = moon
                .position
                .dimensions
                .iter()
                .map(|d| d.abs())
                .sum::<i64>();
            let vel = moon
                .velocity
                .dimensions
                .iter()
                .map(|d| d.abs())
                .sum::<i64>();
            pos * vel
        })
        .sum()
}

#[allow(dead_code)]
fn count_cyclic_steps(moons: &[Moon]) -> i64 {
    let mut result = 1;
    for d in 0..3 {
        let moons = moons.to_vec();
        let get_moon_state =
            |moon: &Moon| (moon.position.dimensions[d], moon.velocity.dimensions[d]);

        let start_state: Vec<_> = moons.iter().map(get_moon_state).collect();

        let step_count = itertools::iterate(moons, |moons| step(moons))
            .skip(1)
            .take_while(|moons| {
                !itertools::equal(
                    moons.iter().map(get_moon_state),
                    start_state.iter().cloned(),
                )
            })
            .count() as i64;
        result = util::lcm(result, step_count + 1);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn point_test() {
        assert_eq!("<x=-1, y=0, z=2>".parse(), Ok(Point3D::new(-1, 0, 2)));
    }

    #[test]
    fn test_1() {
        let mut moons: Vec<_> = "<x=-1, y=0, z=2>\n\
                                 <x=2, y=-10, z=-7>\n\
                                 <x=4, y=-8, z=8>\n\
                                 <x=3, y=5, z=-1>"
            .split('\n')
            .filter_map(|s| s.parse::<Point3D>().ok().map(Moon::new))
            .collect();

        for _ in 0..10 {
            moons = step(&moons);
        }

        assert_eq!(compute_energy(&moons), 179);
    }

    #[test]
    fn test_2() {
        let mut moons: Vec<_> = "<x=-8, y=-10, z=0>\n\
                                 <x=5, y=5, z=10>\n\
                                 <x=2, y=-7, z=3>\n\
                                 <x=9, y=-8, z=-3>"
            .split('\n')
            .filter_map(|s| s.parse::<Point3D>().ok().map(Moon::new))
            .collect();

        for _ in 0..100 {
            dbg!(&moons[0]);
            moons = step(&moons);
        }

        assert_eq!(compute_energy(&moons), 1940);
    }

    #[test]
    fn part_1() {
        let mut moons: Vec<_> = util::input(12)
            .filter_map(|s| s.parse::<Point3D>().ok().map(Moon::new))
            .collect();

        for _ in 0..1000 {
            moons = step(&moons);
        }

        assert_eq!(Some(compute_energy(&moons)), util::answer(12, 1));
    }

    #[test]
    fn test_2_1() {
        let moons: Vec<_> = "<x=-8, y=-10, z=0>\n\
                             <x=5, y=5, z=10>\n\
                             <x=2, y=-7, z=3>\n\
                             <x=9, y=-8, z=-3>"
            .split('\n')
            .filter_map(|s| s.parse::<Point3D>().ok().map(Moon::new))
            .collect();
        assert_eq!(count_cyclic_steps(&moons), 4_686_774_924);
    }

    #[test]
    fn part_2() {
        let moons: Vec<_> = util::input(12)
            .filter_map(|s| s.parse::<Point3D>().ok().map(Moon::new))
            .collect();
        assert_eq!(Some(count_cyclic_steps(&moons)), util::answer(12, 2));
    }
}
