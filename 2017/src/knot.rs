use itertools::Itertools;

pub fn rotate_list(lengths: &[usize], rounds: u8) -> Vec<u8> {
    let mut position: usize = 0;
    let mut skip_size: usize = 0;

    let mut list: Vec<_> = (0..256u16).map(|n| n as u8).collect();

    let list_len = list.len();
    for _ in 0..rounds {
        for length in lengths.iter() {
            list = list.iter()
                .enumerate()
                .map(|(index, value)| {
                    let relative_index = (list_len + index - position) % list_len;
                    if relative_index < *length {
                        list[(position + *length - relative_index - 1) % list_len]
                    } else {
                        *value
                    }
                })
                .collect();

            position = (position + length + skip_size) % list_len;
            skip_size += 1;
        }
    }

    list
}

pub fn compute_hash(input: &str) -> String {
    compute_hash_bytes(input)
        .iter()
        .map(|n| format!("{:02x}", n))
        .collect()
}

pub fn compute_hash_bytes(input: &str) -> Vec<u8> {
    let mut lengths: Vec<_> = input.trim().chars().map(|ch| ch as usize).collect();
    lengths.extend(&[17, 31, 73, 47, 23]);
    let sparse_hash = rotate_list(&lengths, 64);
    sparse_hash
        .into_iter()
        .map(|n| n as u8)
        .chunks(16)
        .into_iter()
        .map(|chunk| chunk.fold1(|r, n| r ^ n).unwrap())
        .collect()
}
