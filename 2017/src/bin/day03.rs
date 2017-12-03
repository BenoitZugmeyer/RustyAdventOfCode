use std::io::{stdin, Read};

fn get_closest_even_root(n: i32) -> i32 {
    let sr = ((n - 1) as f32).sqrt() as i32;
    if sr % 2 == 0 { sr } else { sr + 1 }
}

fn get_position(n: i32) -> (i32, i32) {
    if n == 1 {
        return (0, 0);
    }
    let root = get_closest_even_root(n);
    let position = n - (root - 1) * (root - 1) - 1;
    let position_on_side = position % root - root / 2 + 1;

    match position / root {
        0 => (root / 2, position_on_side),
        1 => (-position_on_side, root / 2),
        2 => (-root / 2, -position_on_side),
        3 => (position_on_side, -root / 2),
        _ => unreachable!(),
    }
}

fn get_index((x, y): (i32, i32)) -> i32 {
    if x == 0 && y == 0 {
        return 1;
    }
    let root = x.abs().max(y.abs()) * 2;
    (root - 1) * (root - 1) + root / 2 +
        if x == root / 2 && y != -root / 2 {
            y
        } else if y == root / 2 {
            root - x
        } else if x == -root / 2 {
            root * 2 - y
        } else if y == -root / 2 {
            root * 3 + x
        } else {
            unreachable!()
        }
}

fn get_next_stress_value(memory: &[i32]) -> i32 {
    let n = memory.len() as i32 + 1;
    if n == 1 {
        return 1;
    }
    let position = get_position(n);

    let mut result = 0;
    for dx in -1..2 {
        for dy in -1..2 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let index = get_index((position.0 + dx, position.1 + dy));
            if index < n {
                result += memory[index as usize - 1];
            }
        }
    }

    result
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).expect(
        "Failed to read stdin",
    );
    let n: i32 = input.trim().parse().expect("Failed to parse input");
    let position = get_position(n);
    println!("Part 1: {}", position.0.abs() + position.1.abs());

    let mut memory: Vec<i32> = Vec::new();

    loop {
        let value = get_next_stress_value(&memory);
        if value > n {
            println!("Part 2: {}", value);
            break;
        }

        memory.push(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_closest_even_root_test() {
        assert_eq!(get_closest_even_root(1), 0);
        assert_eq!(get_closest_even_root(2), 2);
        assert_eq!(get_closest_even_root(9), 2);
        assert_eq!(get_closest_even_root(10), 4);
        assert_eq!(get_closest_even_root(24), 4);
        assert_eq!(get_closest_even_root(25), 4);
        assert_eq!(get_closest_even_root(26), 6);
    }

    #[test]
    fn get_position_test() {
        assert_eq!(get_position(1), (0, 0));
        assert_eq!(get_position(2), (1, 0));
        assert_eq!(get_position(3), (1, 1));
        assert_eq!(get_position(4), (0, 1));
        assert_eq!(get_position(5), (-1, 1));
        assert_eq!(get_position(6), (-1, 0));
        assert_eq!(get_position(7), (-1, -1));
        assert_eq!(get_position(8), (0, -1));
        assert_eq!(get_position(9), (1, -1));
        assert_eq!(get_position(10), (2, -1));
        assert_eq!(get_position(11), (2, 0));
        assert_eq!(get_position(12), (2, 1));
        assert_eq!(get_position(13), (2, 2));
        assert_eq!(get_position(14), (1, 2));
        assert_eq!(get_position(15), (0, 2));
        assert_eq!(get_position(16), (-1, 2));
        assert_eq!(get_position(17), (-2, 2));
        assert_eq!(get_position(18), (-2, 1));
        assert_eq!(get_position(19), (-2, 0));
        assert_eq!(get_position(20), (-2, -1));
        assert_eq!(get_position(21), (-2, -2));
        assert_eq!(get_position(22), (-1, -2));
        assert_eq!(get_position(23), (0, -2));
        assert_eq!(get_position(24), (1, -2));
        assert_eq!(get_position(25), (2, -2));
    }

    #[test]
    fn get_index_test() {
        assert_eq!(get_index((0, 0)), 1);
        assert_eq!(get_index((1, 0)), 2);
        assert_eq!(get_index((1, 1)), 3);
        assert_eq!(get_index((0, 1)), 4);
        assert_eq!(get_index((-1, 1)), 5);
        assert_eq!(get_index((-1, 0)), 6);
        assert_eq!(get_index((-1, -1)), 7);
        assert_eq!(get_index((0, -1)), 8);
        assert_eq!(get_index((1, -1)), 9);
        assert_eq!(get_index((2, -1)), 10);
        assert_eq!(get_index((2, 0)), 11);
        assert_eq!(get_index((2, 1)), 12);
        assert_eq!(get_index((2, 2)), 13);
        assert_eq!(get_index((1, 2)), 14);
        assert_eq!(get_index((0, 2)), 15);
        assert_eq!(get_index((-1, 2)), 16);
        assert_eq!(get_index((-2, 2)), 17);
        assert_eq!(get_index((-2, 1)), 18);
        assert_eq!(get_index((-2, 0)), 19);
        assert_eq!(get_index((-2, -1)), 20);
        assert_eq!(get_index((-2, -2)), 21);
        assert_eq!(get_index((-1, -2)), 22);
        assert_eq!(get_index((0, -2)), 23);
        assert_eq!(get_index((1, -2)), 24);
        assert_eq!(get_index((2, -2)), 25);
    }

    #[test]
    fn get_next_stress_value_test() {
        assert_eq!(get_next_stress_value(&[]), 1);
        assert_eq!(get_next_stress_value(&[1]), 1);
        assert_eq!(get_next_stress_value(&[1, 1]), 2);
        assert_eq!(get_next_stress_value(&[1, 1, 2]), 4);
        assert_eq!(get_next_stress_value(&[1, 1, 2, 4]), 5);
    }
}
