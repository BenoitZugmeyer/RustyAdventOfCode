use crate::intcode::{Program, ProgramResult, Value};
use crate::point::Point;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
fn run_robot(program: &mut Program, start_color: Value) -> HashMap<Point, Value> {
    let mut point = Point::new(0, 0);
    let mut points = HashMap::new();
    let mut direction = 0;
    points.insert(point, start_color);

    while let ProgramResult::NeedInput(output) =
        program.run(&[points.get(&point).cloned().unwrap_or(0)])
    {
        points.insert(point, output[0]);
        direction = if output[1] == 0 {
            direction + 3
        } else {
            direction + 1
        } % 4;
        match direction {
            0 => point.y -= 1,
            1 => point.x += 1,
            2 => point.y += 1,
            3 => point.x -= 1,
            _ => unreachable!(),
        }
    }

    points
}

#[allow(dead_code)]
fn format_image(points: &HashMap<Point, Value>) -> String {
    let white_points: HashSet<_> = points
        .iter()
        .filter_map(|(point, color)| if *color == 1 { Some(point) } else { None })
        .collect();

    let (min_x, max_x) = white_points
        .iter()
        .map(|p| p.x)
        .minmax()
        .into_option()
        .unwrap();
    let (min_y, max_y) = white_points
        .iter()
        .map(|p| p.y)
        .minmax()
        .into_option()
        .unwrap();
    let mut result = String::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            result.push(if white_points.contains(&Point::new(x, y)) {
                '#'
            } else {
                ' '
            });
        }
        result.push('\n');
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ocr::ocr;
    use crate::util;

    fn get_program() -> Program {
        util::input(11).next().expect("No input").parse().unwrap()
    }

    #[test]
    fn part_1() {
        let mut program = get_program();
        let points = run_robot(&mut program, 0);
        assert_eq!(Some(points.len()), util::answer(11, 1));
    }

    #[test]
    fn part_2() {
        let mut program = get_program();
        let points = run_robot(&mut program, 1);
        let formated_image = format_image(&points);
        assert_eq!(Some(ocr(&formated_image)), util::answer(11, 2));
    }
}
