use regex::Regex;
use std::collections::HashMap;
use std::io::{stdin, BufRead};

/// Apply the given regex to a string, and try to parse the first capture as u32
fn parse_int_from_re(re: &Regex, s: &str) -> Option<u32> {
    Some(re.captures(&s)?.get(1)?.as_str().parse().ok()?)
}

/// From a sorted sleep period list, return both the minute with the most overlapping sleep
/// periods, and the count of those sleep periods.
fn most_minute_asleep(sleep_periods: &[(u32, u32)]) -> (u32, usize) {
    let mut ends = Vec::new();
    let mut minute = 0;
    let mut max_periods_asleep = 0;
    for (start, end) in sleep_periods {
        ends = ends
            .into_iter()
            .filter(|&other_end| other_end > start)
            .collect();
        ends.push(end);
        if ends.len() > max_periods_asleep {
            minute = *start;
            max_periods_asleep = ends.len();
        }
    }
    (minute, max_periods_asleep)
}

fn main() {
    // Collect and sort stdin lines
    let mut lines: Vec<String> = stdin().lock().lines().filter_map(|l| l.ok()).collect();
    lines.sort();

    let minutes_re = Regex::new(r"(\d\d)\]").unwrap();
    let guard_id_re = Regex::new(r"#(\d+)").unwrap();

    // Parse the lines and store all sleep periods for each guard
    let mut iterator = lines.iter();
    let mut current_guard_id = 0;
    let mut guards_sleep_periods: HashMap<u32, Vec<(u32, u32)>> = HashMap::new();
    while let Some(line) = iterator.next() {
        // Try to parse the guard id
        if let Some(guard_id) = parse_int_from_re(&guard_id_re, &line) {
            current_guard_id = guard_id
        }
        // Else try to parse the minutes from the current and next line
        else if let (Some(start), Some(end)) = (
            parse_int_from_re(&minutes_re, &line),
            parse_int_from_re(
                &minutes_re,
                &iterator.next().expect("The guard doesn't wake up!"),
            ),
        ) {
            // Add the sleep period to the current guard sleep period list
            guards_sleep_periods
                .entry(current_guard_id)
                .or_insert_with(Vec::new)
                .push((start, end))
        }
    }

    // Sort shifts, and turn guards_sleep_periods into a vector (no need to be a hashmap anymore)
    let guards_sleep_periods: Vec<_> = guards_sleep_periods
        .into_iter()
        .map(|(guard_id, mut sleep_periods)| {
            sleep_periods.sort();
            (guard_id, sleep_periods)
        })
        .collect();

    {
        let (guard_id, sleep_periods) = guards_sleep_periods
            .iter()
            // Find the guard with the most minutes asleep (= sum of the sleep periods duration)
            .max_by_key(|(_, sleep_periods)| sleep_periods.iter().map(|(s, e)| e - s).sum::<u32>())
            // It can only fail if the guards_sleep_periods is empty
            .expect("No guard was given");

        // Find the minute where the guard is the most asleep in general
        let (minute, _) = most_minute_asleep(sleep_periods);

        println!("Part 1: {}", guard_id * minute);
    }

    {
        let (guard_id, (minute, _count)) = guards_sleep_periods
            .iter()
            // For each guard, compute the minute mostly passed asleep, and the shift count where
            // the guard is actually asleep.
            .map(|(guard_id, sleep_periods)| (guard_id, most_minute_asleep(sleep_periods)))
            // Find the guard and minute with the biggest asleep period count
            .max_by_key(|(_guard_id, (_minute, count))| *count)
            // It can only fail if the guards_sleep_periods is empty
            .expect("No guard was given");

        println!("Part 2: {}", guard_id * minute);
    }
}
