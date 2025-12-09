use std::iter;

use itertools::Itertools as _;

type Point = (i64, i64);
type Rectangle = (Point, Point);

fn part_1(lines: impl Iterator<Item = String>) -> i64 {
    let points = parse_points(lines);

    let mut max_area = 0;
    for (i, a) in points.iter().enumerate() {
        for b in points.iter().skip(i + 1) {
            let area = ((a.0 - b.0).abs() + 1) * ((a.1 - b.1).abs() + 1);
            max_area = area.max(max_area);
        }
    }
    max_area
}

fn part_2(lines: impl Iterator<Item = String>) -> i64 {
    let points = parse_points(lines);

    let mut max_area = 0;
    for (i, a) in points.iter().enumerate() {
        for b in points.iter().skip(i + 1) {
            let area = ((a.0 - b.0).abs() + 1) * ((a.1 - b.1).abs() + 1);
            if area > max_area && is_only_red_and_green((*a, *b), &points) {
                max_area = area;
            }
        }
    }
    max_area
}

fn parse_points(lines: impl Iterator<Item = String>) -> Vec<Point> {
    lines
        .map(|s| {
            let (x, y) = s.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

fn is_only_red_and_green(rectangle: Rectangle, points: &[Point]) -> bool {
    let (ra, rb) = normalize_rectangle(rectangle);

    fn is_point_in_rectangle((x, y): Point, ((rax, ray), (rbx, rby)): Rectangle) -> bool {
        rax < x && x < rbx && ray < y && y < rby
    }

    fn normalize_rectangle(((x1, y1), (x2, y2)): Rectangle) -> Rectangle {
        ((x1.min(x2), y1.min(y2)), (x1.max(x2), y1.max(y2)))
    }

    points
        .iter()
        .chain(iter::once(&points[0]))
        .tuple_windows()
        .all(|(pa, pb)| {
            let (pa, pb) = normalize_rectangle((*pa, *pb));
            !(is_point_in_rectangle(pa, ((i64::MIN, ra.1), rb))
                && is_point_in_rectangle(pb, (ra, (i64::MAX, rb.1))))
                && !(is_point_in_rectangle(pa, ((ra.0, i64::MIN), rb))
                    && is_point_in_rectangle(pb, (ra, (rb.0, i64::MAX))))
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;
    const DAY: u8 = 9;

    #[test]
    fn is_only_red_and_green_test() {
        assert!(is_only_red_and_green(((5, 5), (10, 10)), &[(0, 1), (1, 1)]));
        assert!(is_only_red_and_green(
            ((5, 5), (10, 10)),
            &[(5, 5), (5, 10)]
        ));
        assert!(!is_only_red_and_green(
            ((5, 5), (10, 10)),
            &[(6, 1), (6, 6)]
        ));
        assert!(!is_only_red_and_green(
            ((5, 5), (10, 10)),
            &[(6, 1), (6, 11)]
        ));
        assert!(!is_only_red_and_green(
            ((5, 5), (10, 10)),
            &[(6, 1), (6, 10)]
        ));
    }

    #[test]
    fn test1() {
        assert_eq!(part_1(util::example(DAY, 1)), 50);
    }

    #[test]
    fn test2() {
        assert_eq!(part_2(util::example(DAY, 1)), 24);
    }

    #[test]
    fn part_1_test() {
        assert_eq!(Some(part_1(util::input(DAY))), util::answer(DAY, 1));
    }

    #[test]
    fn part_2_test() {
        assert_eq!(Some(part_2(util::input(DAY))), util::answer(DAY, 2));
    }
}
