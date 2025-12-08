type Box = (i64, i64, i64);

fn part_1(lines: impl Iterator<Item = String>, max: usize) -> u32 {
    let (boxes, distances) = parse_boxes_and_sorted_distances(lines);

    let mut circuits: Vec<usize> = boxes.iter().enumerate().map(|(i, _)| i).collect();

    for (i, j, _) in distances.into_iter().take(max) {
        let connected_circuit = circuits[i].min(circuits[j]);
        let removed_circuit = circuits[i].max(circuits[j]);
        for circuit in circuits.iter_mut() {
            if circuit == &removed_circuit {
                *circuit = connected_circuit;
            }
        }
    }

    circuits.sort();

    let mut circuits_counts: Vec<u32> = circuits
        .iter()
        .scan((0, 0u32), |(previous_circuit, count), circuit| {
            if *previous_circuit == *circuit {
                *count += 1;
                Some(None)
            } else {
                let result = Some(*count);
                *previous_circuit = *circuit;
                *count = 1;
                Some(result)
            }
        })
        .flatten()
        .collect();

    circuits_counts.sort();

    circuits_counts.iter().rev().take(3).product::<u32>()
}

fn part_2(lines: impl Iterator<Item = String>) -> u32 {
    let (boxes, distances) = parse_boxes_and_sorted_distances(lines);

    let mut circuits: Vec<usize> = boxes.iter().enumerate().map(|(i, _)| i).collect();

    for (i, j, _) in distances.into_iter() {
        let connected_circuit = circuits[i].min(circuits[j]);
        let removed_circuit = circuits[i].max(circuits[j]);
        let mut all_0 = true;
        for circuit in circuits.iter_mut() {
            if circuit == &removed_circuit {
                *circuit = connected_circuit;
            }
            if *circuit != 0 {
                all_0 = false;
            }
        }
        if all_0 {
            return (boxes[i].0 * boxes[j].0) as u32;
        }
    }

    unreachable!()
}

fn parse_boxes_and_sorted_distances(
    lines: impl Iterator<Item = String>,
) -> (Vec<Box>, Vec<(usize, usize, i64)>) {
    let boxes: Vec<Box> = lines
        .map(|line| {
            let (x, yz) = line.split_once(',').unwrap();
            let (y, z) = yz.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap())
        })
        .collect();

    let mut distances = Vec::new();
    for (i, (x1, y1, z1)) in boxes.iter().enumerate() {
        for (j, (x2, y2, z2)) in boxes.iter().enumerate().skip(i + 1) {
            let distance = (x1 - x2).pow(2) + (y1 - y2).pow(2) + (z1 - z2).pow(2);
            distances.push((i, j, distance));
        }
    }

    distances.sort_by_key(|(_, _, d)| *d);

    (boxes, distances)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;
    const DAY: u8 = 8;

    #[test]
    fn test1() {
        assert_eq!(part_1(util::example(DAY, 1), 10), 40);
    }

    #[test]
    fn test2() {
        assert_eq!(part_2(util::example(DAY, 1)), 25272);
    }

    #[test]
    fn part_1_test() {
        assert_eq!(Some(part_1(util::input(DAY), 1000)), util::answer(DAY, 1));
    }

    #[test]
    fn part_2_test() {
        assert_eq!(Some(part_2(util::input(DAY))), util::answer(DAY, 2));
    }
}
