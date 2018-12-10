use itertools::Itertools;
use regex::Regex;
use std::collections::HashSet;
use std::io::{stdin, BufRead};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

fn print(points: &[Point]) {
    let (min_x, max_x) = points.iter().map(|p| p.x).minmax().into_option().unwrap();
    let (min_y, max_y) = points.iter().map(|p| p.y).minmax().into_option().unwrap();
    let points_set: HashSet<_> = points.iter().map(|p| (p.x, p.y)).collect();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if points_set.contains(&(x, y)) {
                print!("# ");
            } else {
                print!(". ");
            }
        }
        println!();
    }
}

fn main() {
    let re =
        Regex::new(r"position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>").unwrap();
    let mut points: Vec<Point> = stdin()
        .lock()
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| {
            let caps = re.captures(&l)?;
            Some(Point {
                x: caps.get(1)?.as_str().parse().ok()?,
                y: caps.get(2)?.as_str().parse().ok()?,
                dx: caps.get(3)?.as_str().parse().ok()?,
                dy: caps.get(4)?.as_str().parse().ok()?,
            })
        })
        .collect();

    let mut width = i32::max_value();
    for time in 0.. {
        let (min_x, max_x) = points
            .iter()
            .map(|p| p.x + p.dx)
            .minmax()
            .into_option()
            .unwrap();
        let new_width = max_x - min_x;
        if new_width > width {
            println!("Time: {}", time);
            print(&points);
            break;
        }
        width = new_width;
        for point in &mut points {
            point.x += point.dx;
            point.y += point.dy;
        }
    }
}
