extern crate crypto;
extern crate itertools;
use std::io::stdin;
use std::io::Read;
use std::iter;
use std::char;
use crypto::md5::Md5;
use crypto::digest::Digest;

fn has_n_of_a_kind(s: &str, n: usize) -> Option<char> {
    let mut current_ch = None;
    let mut current_count = 0;

    for ch in s.chars() {
        if current_ch == Some(ch) {
            current_count += 1;
            if current_count == n - 1 {
                return Some(ch);
            }
        } else {
            current_ch = Some(ch);
            current_count = 0;
        }
    }

    None
}

fn find_keys_index<B>(hashes: B) -> u32
    where B: Iterator<Item = String>
{
    let mut index = 0;
    let mut count = 0;
    let mut iter = itertools::multipeek(hashes);

    #[allow(while_let_on_iterator)]
    while let Some(h) = iter.next() {
        if let Some(ch) = has_n_of_a_kind(&h, 3) {
            let five_of_a_kind: String = iter::repeat(ch).take(5).collect();
            if (0..1000).any(|_| iter.peek().unwrap().contains(&five_of_a_kind)) {
                count += 1;
                if count == 64 {
                    return index;
                }
            }
        }
        index += 1;
    }

    unreachable!();
}

fn hash(salt: &[u8], index: u32) -> String {
    let mut md5 = Md5::new();
    md5.input(salt);
    md5.input_str(&format!("{}", index));
    md5.result_str()
}

fn stretched_hash(salt: &[u8], index: u32) -> String {
    let mut md5 = Md5::new();
    md5.input(salt);
    md5.input_str(&format!("{}", index));
    (0..2016).fold(md5.result_str(), |previous, _| {
        md5.reset();
        md5.input_str(&previous);
        md5.result_str()
    })
}

fn main() {

    let salt: Vec<_> = stdin()
        .bytes()
        .filter_map(|b| b.ok())
        .filter(|b| b != &b'\n')
        .collect();

    println!("Part 1: {}",
             find_keys_index((0..).map(|index| hash(&salt, index))));

    println!("Part 2: {}",
             find_keys_index((0..).map(|index| stretched_hash(&salt, index))));
}
