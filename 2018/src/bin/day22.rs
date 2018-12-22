use itertools::iproduct;
use regex::Regex;
use std::io::{stdin, Read};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coordinates {
    x: usize,
    y: usize,
}

impl std::convert::Into<Coordinates> for (usize, usize) {
    fn into(self) -> Coordinates {
        Coordinates {
            x: self.0,
            y: self.1,
        }
    }
}

fn read_next<'a, T: std::str::FromStr>(
    iter: &mut impl Iterator<Item = regex::Captures<'a>>,
) -> Option<T> {
    iter.next()?.get(0)?.as_str().parse().ok()
}

fn compute_geologic_level(
    cache: &mut [Vec<Option<usize>>],
    depth: usize,
    target: Coordinates,
    region: Coordinates,
) -> usize {
    if let Some(level) = cache[region.y][region.x] {
        level
    } else {
        let level = (if region == (Coordinates { x: 0, y: 0 }) || region == target {
            0
        } else if region.y == 0 {
            (region.x * 16807)
        } else if region.x == 0 {
            (region.y * 48271)
        } else {
            (compute_geologic_level(
                cache,
                depth,
                target,
                Coordinates {
                    x: region.x - 1,
                    y: region.y,
                },
            )
            .checked_mul(compute_geologic_level(
                cache,
                depth,
                target,
                Coordinates {
                    x: region.x,
                    y: region.y - 1,
                },
            )))
            .expect("Failed to mul")
        } + depth)
            % 20183;

        cache[region.y][region.x] = Some(level);
        level
    }
}

#[allow(dead_code)]
fn print(depth: usize, target: Coordinates) {
    let mut cache = vec![vec![None; target.x + 1]; target.y + 1];
    for y in 0..=target.y {
        for x in 0..=target.x {
            print!(
                "{}",
                if x == 0 && y == 0 {
                    'M'
                } else if x == target.x && y == target.y {
                    'T'
                } else {
                    match compute_geologic_level(&mut cache, depth, target, (x, y).into()) % 3 {
                        0 => '.',
                        1 => '=',
                        2 => '|',
                        _ => unreachable!(),
                    }
                }
            );
        }
        println!();
    }
}

fn compute_risk_level(depth: usize, target: Coordinates) -> usize {
    let mut cache = vec![vec![None; target.x + 1]; target.y + 1];
    iproduct!(0..=target.x, 0..=target.y)
        .map(|c| (compute_geologic_level(&mut cache, depth, target, c.into()) + depth) % 3)
        .sum::<usize>()
}

#[allow(dead_code)]
fn walk(
    cache: &mut [Vec<(usize, usize)>],
    region_types: &[Vec<usize>],
    position: Coordinates,
    tool: usize,
    duration: usize,
) {
    let region_type = if let Some(t) = region_types
        .get(position.y)
        .and_then(|row| row.get(position.x))
    {
        *t
    } else {
        return;
    };
    if region_type == tool {
        return;
    }
    let first_tool = (region_type + 1) % 3;
    let second_tool = (region_type + 2) % 3;
    let durations = &mut cache[position.y][position.x];
    let stored_duration = if tool == first_tool {
        &mut durations.0
    } else {
        &mut durations.1
    };
    if *stored_duration <= duration {
        return;
    }
    *stored_duration = duration;

    if duration > 1026 {
        return;
    }
    for (dx, dy) in &[(0, 1), (2, 1), (1, 0), (1, 2)] {
        if position.x + dx > 0 && position.y + dy > 0 {
            let next_position = Coordinates {
                x: position.x + dx - 1,
                y: position.y + dy - 1,
            };
            if let Some(next_region_type) = region_types
                .get(next_position.y)
                .and_then(|row| row.get(next_position.x))
                .cloned()
            {
                let other_tool = if tool == first_tool {
                    second_tool
                } else {
                    first_tool
                };
                if next_region_type != tool {
                    walk(cache, region_types, next_position, tool, duration + 1);
                } else if next_region_type != other_tool {
                    walk(cache, region_types, next_position, other_tool, duration + 8);
                }
            }
        }
    }
}

fn walk_iter(cache: &mut [Vec<(usize, usize)>], region_types: &[Vec<usize>], _target: Coordinates) {
    let mut positions = vec![(Coordinates { x: 0, y: 0 }, 0, 1, 0)];
    while let Some((position, region_type, tool, duration)) = positions.pop() {
        let first_tool = (region_type + 1) % 3;
        let second_tool = (region_type + 2) % 3;
        let durations = &mut cache[position.y][position.x];
        let stored_duration = if tool == first_tool {
            &mut durations.0
        } else {
            &mut durations.1
        };
        if *stored_duration <= duration {
            continue;
        }
        *stored_duration = duration;

        if duration > 1026 {
            continue;
        }

        #[allow(clippy::filter_map)]
        let vec = [(1, 2), (2, 1), (1, 0), (0, 1)]
            .iter()
            .filter(|(dx, dy)| position.x + dx > 0 && position.y + dy > 0)
            .map(|(dx, dy)| Coordinates {
                x: position.x + dx - 1,
                y: position.y + dy - 1,
            });

        for next_position in vec {
            if let Some(next_region_type) = region_types
                .get(next_position.y)
                .and_then(|row| row.get(next_position.x))
                .cloned()
            {
                let other_tool = if tool == first_tool {
                    second_tool
                } else {
                    first_tool
                };
                if next_region_type != tool {
                    positions.push((next_position, next_region_type, tool, duration + 1));
                } else if next_region_type != other_tool {
                    positions.push((next_position, next_region_type, other_tool, duration + 8));
                }
            }
        }
    }
}

fn compute_rescue_duration(depth: usize, target: Coordinates) -> usize {
    let max_x = target.x + 100;
    let max_y = target.y + 100;
    let mut cache = vec![vec![None; max_x]; max_y];
    let region_types: Vec<Vec<_>> = (0..max_y)
        .map(|y| {
            (0..max_x)
                .map(|x| {
                    (compute_geologic_level(&mut cache, depth, target, (x, y).into()) + depth) % 3
                })
                .collect()
        })
        .collect();

    let mut cache = vec![vec![(usize::max_value(), usize::max_value()); max_x]; max_y];
    // walk(&mut cache, &region_types, Coordinates { x: 0, y: 0 }, 1, 0);
    walk_iter(&mut cache, &region_types, target);
    let (duration_lamp, duration_climbing_gear) = cache[target.y][target.x];
    std::cmp::min(duration_lamp, duration_climbing_gear + 7)
}

// Region types
// 0 rocky (.)
// 1 wet (=)
// 2 narrow (|)
//
// Tools:
// 0 neither
// 1 torch
// 2 climbing gear

fn main() {
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");
    let re = Regex::new(r"\d+").unwrap();

    let mut captures = re.captures_iter(&input);
    let depth: usize = read_next(&mut captures).unwrap();
    let target = Coordinates {
        x: read_next(&mut captures).unwrap(),
        y: read_next(&mut captures).unwrap(),
    };
    println!("Part 1: {}", compute_risk_level(depth, target));
    println!("Part 2: {}", compute_rescue_duration(depth, target));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(compute_risk_level(510, Coordinates { x: 10, y: 10 }), 114);
    }

    #[test]
    fn test_part_2_1() {
        assert_eq!(compute_rescue_duration(510, Coordinates { x: 0, y: 1 }), 1);
    }

    #[test]
    fn test_part_2_2() {
        assert_eq!(compute_rescue_duration(510, Coordinates { x: 1, y: 1 }), 2);
    }

    #[test]
    fn test_part_2_3() {
        assert_eq!(compute_rescue_duration(510, Coordinates { x: 4, y: 1 }), 19);
    }

    #[test]
    fn test_part_2_10() {
        assert_eq!(
            compute_rescue_duration(510, Coordinates { x: 10, y: 10 }),
            45
        );
    }

}
