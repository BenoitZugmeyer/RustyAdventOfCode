use std::io::stdin;
use std::io::Read;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Tile {
    Trap,
    Safe,
}

type Row = Vec<Tile>;


fn generate_room(first_row: Row) -> Box<Iterator<Item = Row>> {
    Box::new((0..).scan(first_row, |next_row, _| {
        let new_row: Vec<_> = (0..next_row.len())
            .map(|column| {
                let left = if column > 0 { next_row[column - 1] } else { Tile::Safe };
                let right =
                    if column < next_row.len() - 1 { next_row[column + 1] } else { Tile::Safe };
                if left == right { Tile::Safe } else { Tile::Trap }
            })
            .collect();
        Some(std::mem::replace(next_row, new_row))
    }))
}

fn main() {

    let first_row: Vec<_> = stdin()
        .bytes()
        .filter_map(|l| l.ok())
        .filter_map(|b| match b {
            b'.' => Some(Tile::Safe),
            b'^' => Some(Tile::Trap),
            _ => None,
        })
        .collect();

    let safe_tile_count = generate_room(first_row.clone())
        .take(40)
        .map(|row| row.iter().filter(|t| t == &&Tile::Safe).count())
        .sum::<usize>();

    println!("Part 1: {}", safe_tile_count);

    let safe_tile_count_400k = generate_room(first_row)
        .take(400_000)
        .map(|row| row.iter().filter(|t| t == &&Tile::Safe).count())
        .sum::<usize>();

    println!("Part 2: {}", safe_tile_count_400k);
}
