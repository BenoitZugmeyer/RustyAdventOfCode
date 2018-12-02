extern crate crypto;
use crypto::digest::Digest;
use crypto::md5::Md5;
use std::char;
use std::io::stdin;
use std::io::Read;
use std::io::Write;

fn main() {
    let salt: Vec<_> = stdin()
        .bytes()
        .filter_map(|b| b.ok())
        .filter(|b| b != &b'\n')
        .collect();

    let mut md5 = Md5::new();
    let mut result = vec![0; md5.output_bytes()];
    let mut formated_index = Vec::new();

    let hashes = (0..).filter_map(|index| {
        formated_index.clear();
        write!(&mut formated_index, "{}", index).unwrap();

        md5.reset();
        md5.input(&salt);
        md5.input(&formated_index);
        md5.result(&mut result);

        if result[0] == 0 && result[1] == 0 && result[2] & 0xf0 == 0 {
            Some((
                char::from_digit(u32::from(result[2]) & 0x0f, 16).unwrap(),
                char::from_digit(u32::from(result[3] >> 4) & 0x0f, 16).unwrap(),
            ))
        } else {
            None
        }
    });

    let mut password1 = String::new();
    let mut password2 = vec![None; 8];
    let mut password1_ok = false;
    let mut password2_ok = false;
    for (ch1, ch2) in hashes {
        if !password1_ok {
            password1.push(ch1);
        }

        if !password2_ok {
            let position = (ch1 as u8 - b'0') as usize;
            if position < 8 && password2[position].is_none() {
                password2[position] = Some(ch2);
            }
        }

        password1_ok = password1.len() == salt.len();
        password2_ok = password2.iter().all(|a| a.is_some());
        if password1_ok && password2_ok {
            break;
        }
    }
    println!("Part 1: {}", password1);
    println!(
        "Part 2: {}",
        password2.iter().map(|a| a.unwrap()).collect::<String>()
    );
}
