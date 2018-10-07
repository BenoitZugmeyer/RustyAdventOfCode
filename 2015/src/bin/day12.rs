extern crate serde_json;

use serde_json::Value;
use std::io;
use std::io::BufRead;

fn sum_numbers(value: &Value) -> i64 {
    match *value {
        Value::I64(n) => n,
        Value::U64(n) => n as i64,
        Value::F64(n) => n as i64,
        Value::Array(ref vec) => vec.iter().map(sum_numbers).sum(),
        Value::Object(ref map) => map.values().map(sum_numbers).sum(),
        _ => 0,
    }
}

fn sum_non_red_numbers(value: &Value) -> i64 {
    match *value {
        Value::Array(ref vec) => vec.iter().map(sum_non_red_numbers).sum(),
        Value::Object(ref map) => {
            let blacklist = Value::String("red".to_string());
            if map.values().any(|v| v == &blacklist) { 0 }
            else { map.values().map(sum_non_red_numbers).sum() }
        },
        ref value => sum_numbers(value),
    }
}

fn main() {

    let stdin = io::stdin();

    let (sum, non_red_sum) = stdin.lock().lines()
        .filter_map(|r| r.ok())
        .filter_map(|ref line| serde_json::from_str(line).ok())
        .map(|ref json| (sum_numbers(json), sum_non_red_numbers(json)))
        .fold((0, 0), |(total_n, total_non_red_n), (n, non_red_n)| {
            (total_n + n, total_non_red_n + non_red_n)
        });

    println!("Total: {}", sum);
    println!("Non red total: {}", non_red_sum);
}
