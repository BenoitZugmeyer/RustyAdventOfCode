use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).expect(
        "Failed to read stdin",
    );
    let steps_count: usize = input.trim().parse().unwrap();
    {
        let max_steps = 2018;

        let mut buffer: Vec<usize> = Vec::with_capacity(max_steps);
        buffer.push(0);
        let mut position = 0;

        for value in 1..max_steps {
            position = (position + steps_count) % value + 1;
            buffer.insert(position, value);
        }

        println!("Part 1: {}", buffer[(position + 1) % buffer.len()]);
    }

    {
        let max_steps = 50_000_001;

        let mut next_to_zero_value = 0;
        let mut position = 0;

        for value in 1..max_steps {
            position = (position + steps_count) % value + 1;
            if position == 1 {
                next_to_zero_value = value;
            }
        }
        println!("Part 2: {}", next_to_zero_value);
    }
}
