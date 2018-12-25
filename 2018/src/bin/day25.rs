use std::io::{stdin, BufRead};

type Coords = (isize, isize, isize, isize);

fn distance(a: &Coords, b: &Coords) -> isize {
    (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs() + (a.3 - b.3).abs()
}

fn main() {
    let mut constellations: Vec<Vec<Coords>> = Vec::new();
    stdin()
        .lock()
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| -> Option<Coords> {
            let mut parts = l.split(',');
            Some((
                parts.next()?.parse().ok()?,
                parts.next()?.parse().ok()?,
                parts.next()?.parse().ok()?,
                parts.next()?.parse().ok()?,
            ))
        })
        .for_each(|coords| {
            #[allow(clippy::filter_map)]
            let indexes: Vec<_> = constellations
                .iter()
                .enumerate()
                .filter(|&(_, constellation)| {
                    constellation
                        .iter()
                        .any(|other| distance(&coords, other) <= 3)
                })
                .map(|(index, _)| index)
                .collect();

            if indexes.is_empty() {
                constellations.push(vec![coords]);
            } else {
                let target = indexes[0];
                constellations[target].push(coords);
                for index in indexes.into_iter().skip(1).rev() {
                    let a = constellations.remove(index);
                    constellations[target].extend(a);
                }
            }
        });

    println!("{}", constellations.len());
}
