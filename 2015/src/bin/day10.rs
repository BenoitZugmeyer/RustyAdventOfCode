use std::io;

#[allow(dead_code)]
fn transform_no_iter(input: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();

    let mut index = 0;
    let len = input.len();
    while index < len {
        let lead = input[index];
        let mut count = 1;
        index += 1;
        while index < len && input[index] == lead {
            count += 1;
            index += 1;
        }
        output.push(count as u8);
        output.push(lead);
    }
    output
}

fn transform(mut input: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();

    while let Some(lead) = input.get(0) {
        let count = input.iter().take_while(|c| *c == lead).count();
        output.push(count as u8);
        output.push(*lead);
        input = &input[count..];
    }
    output
}

fn main() {
    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str).unwrap();
    let mut input: Vec<_> = input_str
        .chars()
        .filter_map(|ch| ch.to_digit(10))
        .map(|d| d as u8)
        .collect();

    for _ in 0..40 {
        input = transform(&input);
    }
    println!("40 times: {}", input.len());
    for _ in 0..10 {
        input = transform(&input);
    }
    println!("50 times: {}", input.len());
}
