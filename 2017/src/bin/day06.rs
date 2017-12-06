use std::io::{stdin, Read};
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).expect(
        "Failed to read stdin",
    );
    let mut banks: Vec<u16> = input
        .trim()
        .split('\t')
        .flat_map(|n| n.parse().ok())
        .collect();
    let bank_count = banks.len() as u16;
    let mut configurations: HashMap<Vec<u16>, usize> = HashMap::new();
    let mut count = 0;
    while !configurations.contains_key(&banks) {
        let (max_index, max_blocks) = banks
            .iter()
            .cloned()
            .enumerate()
            .max_by(|&(index_a, blocks_a), &(index_b, blocks_b)| {
                blocks_a.cmp(&blocks_b).then(
                    index_a.cmp(&index_b).reverse(),
                )
            })
            .unwrap();
        let new_banks = banks
            .iter()
            .enumerate()
            .map(|(index, blocks)| {
                let base_part = if index == max_index { 0 } else { *blocks };
                let equal_part = max_blocks / bank_count;
                let add_part = if (bank_count + index as u16 - max_index as u16 - 1) %
                    bank_count < max_blocks % bank_count
                {
                    1
                } else {
                    0
                };
                base_part + equal_part + add_part
            })
            .collect();
        configurations.insert(banks, count);
        banks = new_banks;
        count += 1;
    }
    println!("Part 1: {}", count);
    println!("Part 2: {}", count - configurations[&banks]);
}
