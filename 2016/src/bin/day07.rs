extern crate itertools;
use itertools::Itertools;

use std::collections::HashSet;
use std::io::stdin;
use std::io::Read;

fn has_abba(s: &str) -> bool {
    s.chars()
        .tuple_windows()
        .any(|(a, b, c, d)| a == d && b == c && a != b)
}

fn get_aba(s: &str) -> Vec<(char, char, char)> {
    s.chars()
        .tuple_windows()
        .filter(|&(a, b, c)| a == c && a != b)
        .collect()
}

fn support_tls(ip: &[String]) -> bool {
    let mut result = false;
    for (index, chunk) in ip.iter().enumerate() {
        if index % 2 == 1 {
            if has_abba(chunk) {
                return false;
            }
        } else if !result {
            result = has_abba(chunk);
        }
    }
    result
}

fn support_ssl(ip: &[String]) -> bool {
    let mut abas = HashSet::new();
    for (index, chunk) in ip.iter().enumerate() {
        if index % 2 == 0 {
            for t in get_aba(chunk) {
                abas.insert(t);
            }
        }
    }
    for (index, chunk) in ip.iter().enumerate() {
        if index % 2 == 1 {
            for &(a, b, _) in &abas {
                if chunk.contains(&format!("{}{}{}", b, a, b)) {
                    return true;
                }
            }
        }
    }
    false
}

fn main() {
    let (count_tls, count_ssl) = stdin()
        .bytes()
        .filter_map(|b| b.ok())
        .map(|b| b as char)
        .batching(|it| {
            let s: Vec<_> = it
                .take_while(|ch| ch != &'\n')
                .batching(|it| {
                    let s: String = it.take_while(|ch| ch != &'[' && ch != &']').collect();
                    if s.is_empty() {
                        None
                    } else {
                        Some(s)
                    }
                })
                .collect();
            if s.is_empty() {
                None
            } else {
                Some(s)
            }
        })
        .fold((0, 0), |(counttls, countssl), ip| {
            (
                counttls + if support_tls(&ip) { 1 } else { 0 },
                countssl + if support_ssl(&ip) { 1 } else { 0 },
            )
        });

    println!("Part 1: {}", count_tls);
    println!("Part 2: {}", count_ssl);
}
