extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;
use std::io;
use std::io::BufRead;

fn find(secret: &[u8], number_of_zeroes: u8) -> u32 {
    let mut result: Vec<u8> = vec![0; 16];
    let mut md5 = Md5::new();
    let number_of_bytes = ((number_of_zeroes + 1) / 2) as usize;
    let mut zeroes = vec![0xffu8; number_of_bytes];
    if number_of_zeroes % 2 == 1 {
        if let Some(last) = zeroes.last_mut() {
            *last = 0xf0;
        }
    }

    (1..)
        .find(|n| {
            md5.input(secret);
            md5.input(n.to_string().as_bytes());
            md5.result(&mut result);
            md5.reset();
            zeroes.iter().zip(&result).all(|(z, r)| z & r == 0)
        })
        .unwrap()
}

fn main() {
    let line = io::stdin().lock().lines().next().unwrap().unwrap();
    let secret = line.as_bytes();
    println!("5 zeroes result: {}", find(&secret, 5));
    println!("6 zeroes result: {}", find(&secret, 6));
}
