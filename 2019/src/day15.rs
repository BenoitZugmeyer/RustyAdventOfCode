use crate::intcode::{Program, ProgramResult};
use crate::point::Point;
use std::collections::HashMap;

fn discover(
    program: &Program,
    position: Point,
    distance_from_start: u32,
    discoved_places: &mut HashMap<Point, u32>,
) -> Option<(Program, Point)> {
    let min_distance = discoved_places.entry(position).or_insert(u32::max_value());
    if distance_from_start > *min_distance {
        return None;
    }
    *min_distance = distance_from_start;

    let mut result = None;

    for direction in 1..=4 {
        let mut dir_program = program.clone();
        match dir_program.run(&[direction]) {
            ProgramResult::Halt(_) => panic!("Program halted Oo"),
            ProgramResult::NeedInput(output) => {
                if output[0] == 1 || output[0] == 2 {
                    let dir_position = match direction {
                        1 => Point::new(position.x, position.y - 1),
                        2 => Point::new(position.x, position.y + 1),
                        3 => Point::new(position.x - 1, position.y),
                        4 => Point::new(position.x + 1, position.y),
                        _ => unreachable!(),
                    };
                    let maybe_position = discover(
                        &dir_program,
                        dir_position,
                        distance_from_start + 1,
                        discoved_places,
                    );
                    if output[0] == 2 {
                        result = Some((dir_program, dir_position));
                    } else {
                        result = result.or(maybe_position);
                    }
                }
            }
        }
    }

    result
}

#[allow(dead_code)]
fn start_discovering(
    program: &Program,
    discoved_places: &mut HashMap<Point, u32>,
) -> Option<(Program, Point)> {
    discover(program, Point::new(0, 0), 0, discoved_places)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn get_program() -> Program {
        util::input(15).next().expect("No input").parse().unwrap()
    }

    #[test]
    fn part_1() {
        let mut discoved_places: HashMap<Point, u32> = HashMap::new();
        let (_, oxygen_position) = start_discovering(&get_program(), &mut discoved_places).unwrap();
        assert_eq!(
            discoved_places.get(&oxygen_position).cloned(),
            util::answer(15, 1)
        );
    }

    #[test]
    fn part_2() {
        let mut discoved_places: HashMap<Point, u32> = HashMap::new();
        let (oxygen_program, _) = start_discovering(&get_program(), &mut discoved_places).unwrap();
        discoved_places.clear();
        start_discovering(&oxygen_program, &mut discoved_places).unwrap();
        assert_eq!(discoved_places.values().cloned().max(), util::answer(15, 2));
    }
}
