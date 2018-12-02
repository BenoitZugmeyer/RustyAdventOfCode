extern crate regex;

use std::io;
use std::io::BufRead;
use regex::Regex;

fn main() {

    let re = Regex::new(r#""|\\\\|\\x|\\"#).unwrap();

    let sum2 = &|(a1, a2), (b1, b2)| (a1 + b1, a2 + b2);

    let (count, encoded_count): (u32, u32) = io::stdin().lock().lines()
        .filter_map(|l| l.ok())
        .map(|ref line| {
            re.captures_iter(line)
                .map(|m| {
                    match m.at(0).unwrap() {
                        r#"""# => (1, 1),
                        r"\\" => (1, 2),
                        r"\" => (0, 1),
                        r"\x" => (3, 1),
                        _ => (0, 0),
                    }
                })
                .fold((0, 2), sum2)
        })
        .fold((0, 0), sum2);


    println!("Extra characters: {}", count);
    println!("Extra characters for encoded string: {}", encoded_count);
}
