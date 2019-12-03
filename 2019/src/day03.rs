use itertools::iproduct;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn distance(self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

type Line = (Point, Point);

/// Check if 'n' is between 'a' and 'b'
fn between(a: i32, b: i32, n: i32) -> bool {
    let min = a.min(b);
    let max = a.max(b);
    min <= n && n <= max
}

/// Compute the intersection between a vertical and a horizontal line
fn compute_intersection_vh(vertical: &Line, horizontal: &Line) -> Option<Point> {
    if between(horizontal.0.x, horizontal.1.x, vertical.0.x)
        && between(vertical.0.y, vertical.1.y, horizontal.0.y)
    {
        Some(Point::new(vertical.0.x, horizontal.0.y))
    } else {
        None
    }
}

/// Compute the intersection between two lines
fn compute_intersection(line1: &Line, line2: &Line) -> Option<Point> {
    if line1.0.x == line1.1.x {
        compute_intersection_vh(line1, line2)
    } else {
        compute_intersection_vh(line2, line1)
    }
}

fn iterate_with_steps<'a, T: Iterator<Item = &'a Line> + 'a + Clone>(
    wire: T,
) -> impl Iterator<Item = (i32, &'a Line)> + 'a + Clone {
    wire.scan(0, |steps, line| {
        let result = (*steps, line);
        *steps += line.0.distance(line.1);
        Some(result)
    })
}

fn wire_intersections<'a>(
    wire1: &'a [Line],
    wire2: &'a [Line],
) -> impl Iterator<Item = (i32, Point)> + 'a {
    iproduct!(
        iterate_with_steps(wire1.iter()),
        iterate_with_steps(wire2.iter())
    )
    .filter_map(|((steps1, line1), (steps2, line2))| {
        compute_intersection(&line1, &line2).map(|intersection| {
            (
                steps1 + line1.0.distance(intersection) + steps2 + line2.0.distance(intersection),
                intersection,
            )
        })
    })
    .filter(|(steps, _)| *steps != 0)
}

#[allow(dead_code)]
fn wire_intersection_min_distance(wire1: &[Line], wire2: &[Line]) -> Option<i32> {
    wire_intersections(wire1, wire2)
        .map(|(_, p)| Point::new(0, 0).distance(p))
        .min()
}

#[allow(dead_code)]
fn wire_intersection_min_steps(wire1: &[Line], wire2: &[Line]) -> Option<i32> {
    wire_intersections(wire1, wire2)
        .map(|(steps, _)| steps)
        .min()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn parse_wire(s: &str) -> Vec<Line> {
        s.split(',')
            .scan(Point::new(0, 0), |previous, path| {
                let distance = path[1..].parse::<i32>().expect("Failed to parse distance");
                let next = match path.chars().nth(0).expect("Failed to parse direction") {
                    'R' => Point::new(previous.x + distance, previous.y),
                    'L' => Point::new(previous.x - distance, previous.y),
                    'U' => Point::new(previous.x, previous.y + distance),
                    'D' => Point::new(previous.x, previous.y - distance),
                    d => panic!("Invalid direction {}", d),
                };
                let result = (*previous, next);
                *previous = next;
                Some(result)
            })
            .collect()
    }

    #[test]
    fn parse_wire_test() {
        assert_eq!(
            parse_wire("R8,U5,L5,D3"),
            vec![
                (Point::new(0, 0), Point::new(8, 0)),
                (Point::new(8, 0), Point::new(8, 5)),
                (Point::new(8, 5), Point::new(3, 5)),
                (Point::new(3, 5), Point::new(3, 2))
            ]
        );
    }

    #[test]
    fn compute_intersection_test() {
        assert_eq!(
            compute_intersection(
                &(Point::new(0, 0), Point::new(0, 10)),
                &(Point::new(-5, 5), Point::new(5, 5))
            ),
            Some(Point::new(0, 5))
        );

        assert_eq!(
            compute_intersection(
                &(Point::new(0, 0), Point::new(0, 10)),
                &(Point::new(5, 5), Point::new(5, 10))
            ),
            None
        );

        assert_eq!(
            compute_intersection(
                &(Point::new(3, 5), Point::new(3, 2)),
                &(Point::new(0, 7), Point::new(6, 7))
            ),
            None
        );

        assert_eq!(
            compute_intersection(
                &(Point::new(3, 5), Point::new(3, 2)),
                &(Point::new(6, 3), Point::new(2, 3))
            ),
            Some(Point::new(3, 3))
        );
    }

    #[test]
    fn wire_intersection_min_distance_test() {
        assert_eq!(
            wire_intersection_min_distance(&parse_wire("R8,U5,L5,D3"), &parse_wire("U7,R6,D4,L4")),
            Some(6)
        );
        assert_eq!(
            wire_intersection_min_distance(
                &parse_wire("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
                &parse_wire("U62,R66,U55,R34,D71,R55,D58,R83")
            ),
            Some(159)
        );
        assert_eq!(
            wire_intersection_min_distance(
                &parse_wire("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
                &parse_wire("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
            ),
            Some(135)
        );
    }

    #[test]
    fn part_1() {
        let wires = util::input(3).collect::<Vec<String>>();
        assert_eq!(
            wire_intersection_min_distance(&parse_wire(&wires[0]), &parse_wire(&wires[1])),
            util::answer(3, 1)
        );
    }

    #[test]
    fn wire_intersection_min_steps_tests() {
        assert_eq!(
            wire_intersection_min_steps(&parse_wire("R8,U5,L5,D3"), &parse_wire("U7,R6,D4,L4")),
            Some(30)
        );
        assert_eq!(
            wire_intersection_min_steps(
                &parse_wire("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
                &parse_wire("U62,R66,U55,R34,D71,R55,D58,R83")
            ),
            Some(610)
        );
        assert_eq!(
            wire_intersection_min_steps(
                &parse_wire("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
                &parse_wire("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
            ),
            Some(410)
        );
    }
    #[test]
    fn part_2() {
        let wires = util::input(3).collect::<Vec<String>>();
        assert_eq!(
            wire_intersection_min_steps(&parse_wire(&wires[0]), &parse_wire(&wires[1])),
            util::answer(3, 2)
        );
    }
}
