fn validate_passports<F: Fn(&str, &str) -> bool, T: Iterator<Item = String>>(
    lines: T,
    is_required_valid_field: F,
) -> usize {
    let mut count = 0;
    lines
        .chain(std::iter::once("".into()))
        .filter_map(|line| {
            if line.is_empty() {
                let is_valid = count == 7;
                count = 0;
                if is_valid {
                    Some(true)
                } else {
                    None
                }
            } else {
                for whole_field in line.split(" ") {
                    let mut it = whole_field.splitn(2, ":");
                    let name = it.next().unwrap();
                    let value = it.next().unwrap();
                    if is_required_valid_field(name, value) {
                        count += 1;
                    }
                }
                None
            }
        })
        .count()
}

#[allow(dead_code)]
fn part_1<T: Iterator<Item = String>>(lines: T) -> usize {
    validate_passports(lines, |name, _| name != "cid")
}

#[allow(dead_code)]
fn part_2<T: Iterator<Item = String>>(lines: T) -> usize {
    validate_passports(lines, |name, value| match name {
        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        "byr" => {
            value.len() == 4
                && value
                    .parse::<u32>()
                    .map(|year| 1920 <= year && year <= 2002)
                    .unwrap_or(false)
        }
        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        "iyr" => {
            value.len() == 4
                && value
                    .parse::<u32>()
                    .map(|year| 2010 <= year && year <= 2020)
                    .unwrap_or(false)
        }
        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        "eyr" => {
            value.len() == 4
                && value
                    .parse::<u32>()
                    .map(|year| 2020 <= year && year <= 2030)
                    .unwrap_or(false)
        }
        // hgt (Height) - a number followed by either cm or in:
        //     If cm, the number must be at least 150 and at most 193.
        //     If in, the number must be at least 59 and at most 76.
        "hgt" => {
            let number = &value[0..value.len() - 2];
            let unit = &value[value.len() - 2..];
            number
                .parse::<u32>()
                .map(|height| match unit {
                    "cm" => 150 <= height && height <= 193,
                    "in" => 59 <= height && height <= 76,
                    _ => false,
                })
                .unwrap_or(false)
        }
        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        "hcl" => {
            value.len() == 7
                && value.chars().next() == Some('#')
                && value
                    .chars()
                    .skip(1)
                    .all(|ch| '0' <= ch && ch <= '9' || 'a' <= ch && ch <= 'f')
        }
        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        "ecl" => matches!(value, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"),
        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        "pid" => value.len() == 9 && value.parse::<u64>().map(|_| true).unwrap_or(false),
        // cid (Country ID) - ignored, missing or not.
        // and other
        _ => false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test() {
        assert_eq!(part_1(util::example(4, 1)), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(part_2(util::example(4, 3)), 0);
    }

    #[test]
    fn test3() {
        assert_eq!(part_2(util::example(4, 4)), 4);
    }

    #[test]
    fn part_1_test() {
        assert_eq!(Some(part_1(util::input(4))), util::answer(4, 1));
    }

    #[test]
    fn part_2_test() {
        assert_eq!(Some(part_2(util::input(4))), util::answer(4, 2));
    }
}
