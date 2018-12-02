use std::io::stdin;
use std::io::Read;

fn read_int<B>(chars: &mut B) -> u64
where
    B: Iterator<Item = char>,
{
    let mut result = 0u64;
    for ch in chars {
        if let Some(n) = ch.to_digit(10) {
            result = result * 10 + u64::from(n);
        } else {
            break;
        }
    }
    result
}

fn decompress_len(s: &str, recurse: bool) -> u64 {
    let mut result = 0;

    let mut chars = s.chars();

    while let Some(ch) = chars.next() {
        if ch == '(' {
            let char_count = read_int(&mut chars);
            let times = read_int(&mut chars);
            if recurse {
                let slice: String = chars.by_ref().take(char_count as usize).collect();
                result += times * decompress_len(&slice, recurse);
            } else if char_count > 0 {
                chars.by_ref().nth(char_count as usize - 1);
                result += times * char_count;
            }
        } else if ch != ' ' {
            result += 1;
        }
    }

    result
}

fn main() {
    let input: String = stdin()
        .bytes()
        .filter_map(|b| b.ok())
        .map(|b| b as char)
        .take_while(|ch| ch != &'\n')
        .collect();

    println!("Part 1: {}", decompress_len(&input, false));
    println!("Part 2: {}", decompress_len(&input, true));
}
