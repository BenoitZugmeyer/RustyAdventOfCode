use std::io::{stdin, Read};

fn distance(point: (i16, i16, i16)) -> i16 {
    (point.0.abs() + point.1.abs() + point.2.abs()) / 2
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).expect(
        "Failed to read stdin",
    );

    let (max_distance, coord) = input.trim().split(',').fold(
        (0i16, (0i16, 0i16, 0i16)),
        |(max_distance, pd), direction| {
            let new_position = match direction {
                "n" => (pd.0, pd.1 + 1, pd.2 - 1),
                "s" => (pd.0, pd.1 - 1, pd.2 + 1),
                "nw" => (pd.0 + 1, pd.1, pd.2 - 1),
                "se" => (pd.0 - 1, pd.1, pd.2 + 1),
                "ne" => (pd.0 - 1, pd.1 + 1, pd.2),
                "sw" => (pd.0 + 1, pd.1 - 1, pd.2),
                _ => unreachable!(),
            };

            (
                std::cmp::max(max_distance, distance(new_position)),
                new_position,
            )
        },
    );

    println!("Part 1: {}", distance(coord));
    println!("Part 2: {}", max_distance);
}
