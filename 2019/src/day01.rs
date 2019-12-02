fn fuel_required(mass: u32) -> u32 {
    (mass / 3).saturating_sub(2)
}

fn fuel_required_with_requirement(mut mass: u32) -> u32 {
    let mut result = 0;
    while mass != 0 {
        mass = fuel_required(mass);
        result += mass;
    }
    result
}

#[allow(dead_code)]
fn total_fuel_required<T: Iterator<Item = u32>>(iterator: T) -> u32 {
    iterator.map(fuel_required).sum()
}

#[allow(dead_code)]
fn total_fuel_required_with_requirement<T: Iterator<Item = u32>>(iterator: T) -> u32 {
    iterator.map(fuel_required_with_requirement).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn fuel_required_test() {
        assert_eq!(fuel_required(1), 0);
        assert_eq!(fuel_required(12), 2);
        assert_eq!(fuel_required(14), 2);
        assert_eq!(fuel_required(1969), 654);
        assert_eq!(fuel_required(100_756), 33583);
    }

    #[test]
    fn fuel_required_with_requirement_test() {
        assert_eq!(fuel_required_with_requirement(12), 2);
        assert_eq!(fuel_required_with_requirement(1969), 966);
        assert_eq!(fuel_required_with_requirement(100_756), 50346);
    }

    #[test]
    fn part_1() {
        assert_eq!(
            Some(u64::from(total_fuel_required(
                util::input(1).flat_map(|line| line.parse())
            ))),
            util::answer(1, 1)
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(
            Some(u64::from(total_fuel_required_with_requirement(
                util::input(1).flat_map(|line| line.parse())
            ))),
            util::answer(1, 2),
        );
    }
}
