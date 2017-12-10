use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let numbers: Vec<_> = input.chars().flat_map(|ch| ch.to_digit(10)).collect();

    let mut part_1 = 0;
    let mut part_2 = 0;
    for (index, n) in numbers.iter().enumerate() {
        if n == &numbers[(index + 1) % numbers.len()] {
            part_1 += n;
        }
        if n == &numbers[(index + numbers.len() / 2) % numbers.len()] {
            part_2 += n;
        }
    }

    println!("Part 1: {:?}", part_1);
    println!("Part 2: {:?}", part_2);
}
