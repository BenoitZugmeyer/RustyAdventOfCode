extern crate regex;

use std::io::{stdin, Read};
use regex::Regex;

fn main() {
    let re = Regex::new(r"\d+").expect("failed to parse regex");
    let mut input = String::new();
    stdin().read_to_string(&mut input).expect(
        "Failed to read stdin",
    );

    let values: Vec<_> = re.captures_iter(&input)
        .flat_map(|cap| cap.get(0).unwrap().as_str().parse::<u64>().ok())
        .collect();

    {
        let mut value_a = values[0];
        let mut value_b = values[1];
        let mask = (1 << 16) - 1;
        let mut count = 0;

        for _ in 0..40_000_000 {
            value_a = (value_a * 16_807) % 2_147_483_647;
            value_b = (value_b * 48_271) % 2_147_483_647;
            if value_a & mask == value_b & mask {
                count += 1;
            }
        }
        println!("Part 1: {}", count);
    }

    {
        let mut value_a = values[0];
        let mut value_b = values[1];
        let mask = (1 << 16) - 1;
        let mut count = 0;

        for _ in 0..5_000_000 {
            loop {
                value_a = (value_a * 16_807) % 2_147_483_647;
                if value_a % 4 == 0 {
                    break;
                }
            }

            loop {
                value_b = (value_b * 48_271) % 2_147_483_647;
                if value_b % 8 == 0 {
                    break;
                }
            }

            if value_a & mask == value_b & mask {
                count += 1;
            }
        }
        println!("Part 2: {}", count);
    }
}
