use crate::point::Point;
use std::collections::HashSet;

fn compute_closest_point(p: Point) -> Point {
    let mut a = p.x.abs().max(p.y.abs());
    let mut b = p.x.abs().min(p.y.abs());
    loop {
        if b == 0 {
            return Point::new(p.x / a, p.y / a);
        }
        let r = a % b;
        a = b;
        b = r;
    }
}

fn count_asteroids_in_sight(map: &HashSet<Point>, origin: Point) -> usize {
    let mut is_in_sight = HashSet::new();
    for target in map {
        if &origin == target {
            continue;
        }

        let relative_point = target.relative_to(origin);
        is_in_sight.insert(compute_closest_point(relative_point));
    }

    is_in_sight.len()
}

#[allow(dead_code)]
fn find_best_asteroid(map: &HashSet<Point>) -> (Point, usize) {
    map.iter()
        .map(|origin| (*origin, count_asteroids_in_sight(&map, *origin)))
        .max_by_key(|(_, count)| *count)
        .unwrap()
}

#[allow(clippy::cast_precision_loss)]
fn compute_angle(p: Point) -> f32 {
    -(p.x as f32).atan2(p.y as f32)
}

#[allow(dead_code)]
fn find_200th_shoted_asteroid(mut map: HashSet<Point>) -> Option<Point> {
    let origin = find_best_asteroid(&map).0;
    map.remove(&origin);

    let mut points: Vec<_> = map.iter().collect();
    points.sort_by(|a, b| {
        let angle_a = compute_angle(a.relative_to(origin));
        let angle_b = compute_angle(b.relative_to(origin));
        angle_a
            .partial_cmp(&angle_b)
            .unwrap()
            .then_with(|| a.distance(origin).cmp(&b.distance(origin)))
    });

    let mut i = 0;
    while !points.is_empty() {
        let mut unreachable_points = Vec::new();
        let mut previous_direction = None;
        for point in points {
            let direction = compute_angle(point.relative_to(origin));
            if previous_direction == Some(direction) {
                unreachable_points.push(point);
            } else {
                i += 1;
                if i == 200 {
                    return Some(*point);
                }
                previous_direction = Some(direction);
            }
        }
        points = unreachable_points;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;
    use std::convert::TryInto;
    use std::f32;

    #[test]
    fn compute_closest_point_test() {
        assert_eq!(compute_closest_point(Point::new(3, 6)), Point::new(1, 2));
        assert_eq!(compute_closest_point(Point::new(-3, 6)), Point::new(-1, 2));
        assert_eq!(compute_closest_point(Point::new(10, 10)), Point::new(1, 1));
        assert_eq!(compute_closest_point(Point::new(0, 10)), Point::new(0, 1));
    }

    fn map_from_str(s: &str) -> HashSet<Point> {
        s.split('\n')
            .enumerate()
            .flat_map(|(y, line)| {
                line.bytes().enumerate().filter_map(move |(x, ch)| {
                    if ch == b'#' {
                        Some(Point::new(x.try_into().unwrap(), y.try_into().unwrap()))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    #[test]
    fn test_1() {
        let map = map_from_str(
            ".#..#\n\
             .....\n\
             #####\n\
             ....#\n\
             ...##",
        );
        assert_eq!(count_asteroids_in_sight(&map, Point::new(3, 4)), 8);
        assert_eq!(find_best_asteroid(&map), (Point::new(3, 4), 8));
    }

    #[test]
    fn test_2() {
        let map = map_from_str(
            "......#.#.\n\
             #..#.#....\n\
             ..#######.\n\
             .#.#.###..\n\
             .#..#.....\n\
             ..#....#.#\n\
             #..#....#.\n\
             .##.#..###\n\
             ##...#..#.\n\
             .#....####",
        );
        assert_eq!(find_best_asteroid(&map), (Point::new(5, 8), 33));
    }

    #[test]
    fn test_3() {
        let map = map_from_str(
            "#.#...#.#.\n\
             .###....#.\n\
             .#....#...\n\
             ##.#.#.#.#\n\
             ....#.#.#.\n\
             .##..###.#\n\
             ..#...##..\n\
             ..##....##\n\
             ......#...\n\
             .####.###.",
        );
        assert_eq!(find_best_asteroid(&map), (Point::new(1, 2), 35));
    }

    #[test]
    fn test_4() {
        let map = map_from_str(
            ".#..#..###\n\
             ####.###.#\n\
             ....###.#.\n\
             ..###.##.#\n\
             ##.##.#.#.\n\
             ....###..#\n\
             ..#.#..#.#\n\
             #..#.#.###\n\
             .##...##.#\n\
             .....#.#..",
        );
        assert_eq!(find_best_asteroid(&map), (Point::new(6, 3), 41));
    }

    #[test]
    fn test_5() {
        let map = map_from_str(
            ".#..##.###...#######\n\
             ##.############..##.\n\
             .#.######.########.#\n\
             .###.#######.####.#.\n\
             #####.##.#.##.###.##\n\
             ..#####..#.#########\n\
             ####################\n\
             #.####....###.#.#.##\n\
             ##.#################\n\
             #####.##.###..####..\n\
             ..######..##.#######\n\
             ####.##.####...##..#\n\
             .#####..#.######.###\n\
             ##...#.##########...\n\
             #.##########.#######\n\
             .####.#.###.###.#.##\n\
             ....##.##.###..#####\n\
             .#.#.###########.###\n\
             #.#.#.#####.####.###\n\
             ###.##.####.##.#..##",
        );
        assert_eq!(find_best_asteroid(&map), (Point::new(11, 13), 210));
    }

    fn get_map() -> HashSet<Point> {
        util::input(10)
            .enumerate()
            .flat_map(|(y, line)| {
                line.into_bytes()
                    .into_iter()
                    .enumerate()
                    .filter_map(move |(x, ch)| {
                        if ch == b'#' {
                            Some(Point::new(x.try_into().unwrap(), y.try_into().unwrap()))
                        } else {
                            None
                        }
                    })
            })
            .collect()
    }

    #[test]
    fn part_1() {
        let map = get_map();
        assert_eq!(Some(find_best_asteroid(&map).1), util::answer(10, 1));
    }

    #[test]
    fn compute_angle_test() {
        let f32_eq = |actual: f32, expected: f32| (actual - expected).abs() <= 0.01;
        assert!(f32_eq(compute_angle(Point::new(0, -1)), -f32::consts::PI));
        assert!(f32_eq(compute_angle(Point::new(0, -2)), -f32::consts::PI));
        assert!(f32_eq(
            compute_angle(Point::new(1, 0)),
            -f32::consts::FRAC_PI_2
        ));
        assert!(f32_eq(compute_angle(Point::new(0, 1)), 0.));
        assert!(f32_eq(
            compute_angle(Point::new(-1, 0)),
            f32::consts::FRAC_PI_2
        ));
        assert!(f32_eq(
            compute_angle(Point::new(-1, -1)),
            3. * f32::consts::FRAC_PI_4
        ));
    }

    #[test]
    fn test_2_1() {
        let map = map_from_str(
            ".#..##.###...#######\n\
             ##.############..##.\n\
             .#.######.########.#\n\
             .###.#######.####.#.\n\
             #####.##.#.##.###.##\n\
             ..#####..#.#########\n\
             ####################\n\
             #.####....###.#.#.##\n\
             ##.#################\n\
             #####.##.###..####..\n\
             ..######..##.#######\n\
             ####.##.####...##..#\n\
             .#####..#.######.###\n\
             ##...#.##########...\n\
             #.##########.#######\n\
             .####.#.###.###.#.##\n\
             ....##.##.###..#####\n\
             .#.#.###########.###\n\
             #.#.#.#####.####.###\n\
             ###.##.####.##.#..##",
        );
        assert_eq!(find_200th_shoted_asteroid(map), Some(Point::new(8, 2)));
    }

    #[test]
    fn part_2() {
        let map = get_map();
        assert_eq!(
            find_200th_shoted_asteroid(map).map(|point| point.x * 100 + point.y),
            util::answer(10, 2)
        );
    }
}
