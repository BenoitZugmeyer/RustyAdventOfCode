extern crate pcre;

use std::io;
use std::io::BufRead;
use pcre::Pcre;

fn main() {
    let mut vowels: Pcre = Pcre::compile(r"[aeiou].*[aeiou].*[aeiou]").unwrap();
    let mut double: Pcre = Pcre::compile(r"(.)\1").unwrap();
    let mut bad: Pcre = Pcre::compile(r"ab|cd|pq|xy").unwrap();

    let mut pairs = Pcre::compile(r"(..).*\1").unwrap();
    let mut repeats = Pcre::compile(r"(.).\1").unwrap();

    let is_nice = &mut |s: &str| vowels.exec(s).is_some() && double.exec(s).is_some() && bad.exec(s).is_none();
    let better_is_nice = &mut |s: &str| pairs.exec(s).is_some() && repeats.exec(s).is_some();

    let (count, better_count) = io::stdin().lock().lines()
        .filter_map(|l| l.ok())
        .fold((0, 0), |(n, bn), ref line| (
            n + if is_nice(line) { 1 } else { 0 },
            bn + if better_is_nice(line) { 1 } else { 0 },
        ));

    println!("Nice strings: {}", count);
    println!("Better nice strings: {}", better_count);
}
