fn part_1(mut lines: impl Iterator<Item = String>) -> u64 {
    let mut beams: Vec<bool> = lines.next().unwrap().chars().map(|c| c == 'S').collect();
    let mut count = 0;
    for line in lines {
        let mut next_beams = vec![false; beams.len()];

        for (i, c) in line
            .chars()
            .zip(&beams)
            .enumerate()
            .filter_map(|(i, (c, b))| b.then_some((i, c)))
        {
            if c == '.' {
                next_beams[i] = true;
            } else {
                count += 1;
                next_beams[i - 1] = true;
                next_beams[i + 1] = true;
            }
        }
        beams = next_beams;
    }

    count
}

fn part_2(mut lines: impl Iterator<Item = String>) -> u64 {
    let mut timelines: Vec<u64> = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| if c == 'S' { 1 } else { 0 })
        .collect();

    for line in lines {
        for i in line
            .chars()
            .enumerate()
            .filter_map(|(i, ch)| (ch == '^').then_some(i))
        {
            timelines[i + 1] += timelines[i];
            timelines[i - 1] += timelines[i];
            timelines[i] = 0;
        }
    }
    timelines.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;
    const DAY: u8 = 7;

    #[test]
    fn test1() {
        assert_eq!(part_1(util::example(DAY, 1)), 21);
    }

    #[test]
    fn test2() {
        assert_eq!(part_2(util::example(DAY, 1)), 40);
    }

    #[test]
    fn part_1_test() {
        assert_eq!(Some(part_1(util::input(DAY))), util::answer(DAY, 1));
    }

    #[test]
    fn part_2_test() {
        assert_eq!(Some(part_2(util::input(DAY))), util::answer(DAY, 2));
    }
}
