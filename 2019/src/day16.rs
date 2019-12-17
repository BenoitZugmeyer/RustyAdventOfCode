use std::iter;

/// Naive approach, this describes simply what's requested, but can be optimized
#[allow(dead_code, clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn run_phase_naive(signal: &[u8]) -> Vec<u8> {
    let pattern = [0, 1, 0, -1];
    (0..signal.len())
        .map(|i| {
            let sum: i32 = signal
                .iter()
                .zip(
                    pattern
                        .iter()
                        .cycle()
                        .flat_map(|p| iter::repeat(p).take(i + 1))
                        .skip(1),
                )
                .map(|(n, p)| i32::from(*n) * p)
                .sum();
            (sum % 10).abs() as u8
        })
        .collect()
}

/// Optimized approach, we skip a lot of computation by reusing the previously computed sums
#[allow(dead_code, clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn run_phase(signal: &[u8]) -> Vec<u8> {
    let mut base_sum = 0;
    let mut res = vec![0; signal.len()];
    for i in (0..signal.len()).rev() {
        base_sum += i32::from(signal[i]);
        let sub_pattern_size = i + 1;
        let pattern_size = sub_pattern_size * 4;
        base_sum -= signal
            .iter()
            .skip(sub_pattern_size * 2 - 1)
            .take(2)
            .cloned()
            .map(i32::from)
            .sum::<i32>();

        let mut sum = base_sum;
        for j in (((sub_pattern_size * 3) - 1)..signal.len()).step_by(pattern_size) {
            sum -= signal[j..]
                .iter()
                .take(sub_pattern_size)
                .cloned()
                .map(i32::from)
                .sum::<i32>();
            sum += signal[j..]
                .iter()
                .skip(sub_pattern_size * 2)
                .take(sub_pattern_size)
                .cloned()
                .map(i32::from)
                .sum::<i32>();
        }
        res[i] = (sum % 10).abs() as u8;
    }
    res
}

/// Overly optimized solution. It only works if the "offset" is greater than 5000 times the length
/// of the input.
#[allow(dead_code)]
fn run_phases_for_10k_input(input: &[u8]) -> Vec<u8> {
    let offset = input
        .iter()
        .take(7)
        .fold(0_usize, |total, &digit| total * 10 + digit as usize);
    let total_size = input.len() * 10_000;
    let needed_size = total_size - offset;
    let mut signal: Vec<_> = input
        .iter()
        .cycle()
        .skip(input.len() - (needed_size % input.len()))
        .take(needed_size)
        .cloned()
        .collect();

    for _ in 0..100 {
        let mut next_signal = vec![0; needed_size];
        let mut sum = 0;
        for i in (0..needed_size).rev() {
            sum = (sum + signal[i]) % 10;
            next_signal[i] = sum;
        }

        signal = next_signal
    }

    signal[..8].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn join<S: std::string::ToString, T: Iterator<Item = S>>(val: T) -> String {
        val.map(|s| s.to_string()).collect()
    }

    fn get_signal() -> Vec<u8> {
        util::input(16)
            .next()
            .unwrap()
            .bytes()
            .map(|b| b - b'0')
            .collect()
    }

    #[test]
    fn test_1_1_naive() {
        let signal = [1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(run_phase_naive(&signal), [4, 8, 2, 2, 6, 1, 5, 8]);
    }

    #[test]
    fn test_1_1() {
        let signal = [1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(run_phase(&signal), [4, 8, 2, 2, 6, 1, 5, 8]);
    }

    #[test]
    fn part_1() {
        let mut signal = get_signal();
        for _ in 0..100 {
            signal = run_phase(&signal);
        }
        assert_eq!(Some(join(signal[..8].iter())), util::answer(16, 1));
    }

    #[test]
    fn test_2_1() {
        let base_signal = [
            0, 3, 0, 3, 6, 7, 3, 2, 5, 7, 7, 2, 1, 2, 9, 4, 4, 0, 6, 3, 4, 9, 1, 5, 6, 5, 4, 7, 4,
            6, 6, 4,
        ];
        assert_eq!(
            run_phases_for_10k_input(&base_signal),
            [8, 4, 4, 6, 2, 0, 2, 6]
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(
            Some(join(run_phases_for_10k_input(&get_signal()).iter())),
            util::answer(16, 2)
        );
    }
}
