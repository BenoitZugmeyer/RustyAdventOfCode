use std::collections::HashMap;
use std::io::stdin;
use std::io::Read;

fn main() {

    let columns = stdin()
        .bytes()
        .filter_map(|b| b.ok())
        .map(|b| b as char)
        .fold((0, Vec::new()), |(mut column_index, mut columns), ch| {
            if ch == '\n' {
                column_index = 0;
            } else {
                if columns.len() <= column_index {
                    columns.push(HashMap::new());
                }
                *columns[column_index].entry(ch).or_insert(0) += 1;
                column_index += 1;
            }
            (column_index, columns)
        })
        .1;

    let message1: String = columns.iter()
        .map(|column| *column.iter().max_by_key(|&(_, count)| count).unwrap().0)
        .collect();

    let message2: String = columns.iter()
        .map(|column| *column.iter().min_by_key(|&(_, count)| count).unwrap().0)
        .collect();

    println!("Part 1: {}", message1);
    println!("Part 2: {}", message2);
}
