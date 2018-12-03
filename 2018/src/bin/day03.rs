use regex::Regex;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::io::{stdin, BufRead};

#[derive(Debug)]
struct Rectangle {
    id: u32,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

fn main() {
    let reg =
        Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").expect("Failed to compile the regex");
    let rectangles: Vec<Rectangle> = stdin()
        .lock()
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|line| {
            reg.captures(&line).and_then(|caps| {
                Some(Rectangle {
                    id: caps.get(1)?.as_str().parse().ok()?,
                    left: caps.get(2)?.as_str().parse().ok()?,
                    top: caps.get(3)?.as_str().parse().ok()?,
                    width: caps.get(4)?.as_str().parse().ok()?,
                    height: caps.get(5)?.as_str().parse().ok()?,
                })
            })
        })
        .collect();

    // Associate coordinates (in inches) with either:
    // * Some(id) if the given square inch has a single rectangle on it
    // * None if the given square inch has more than one rectangle on it
    let mut fabric: HashMap<(u32, u32), Option<u32>> = HashMap::new();

    // The rectangle ids which are still intact
    let mut intact_ids: HashSet<u32> = HashSet::new();

    for rectangle in &rectangles {
        let mut is_intact = true;
        for x in (rectangle.left + 1)..(rectangle.left + 1 + rectangle.width) {
            for y in (rectangle.top + 1)..(rectangle.top + 1 + rectangle.height) {
                match fabric.entry((x, y)) {
                    Entry::Occupied(mut entry) => {
                        // If the square inch is already occupied, mark the current rectangle as
                        // not intact anymore
                        is_intact = false;
                        if let Some(id) = entry.get() {
                            // If the square inch was occupied by a single rectangle, remove
                            // its id from the intact_ids list
                            intact_ids.remove(&id);
                            entry.insert(None);
                        }
                    }
                    Entry::Vacant(entry) => {
                        // The square inch is vacant, store our id in it
                        entry.insert(Some(rectangle.id));
                    }
                }
            }
        }
        // If the rectangle is intact, keep its id in memory
        if is_intact {
            intact_ids.insert(rectangle.id);
        }
    }

    println!(
        "Part 1: {}",
        fabric.values().filter(|v| v.is_none()).count()
    );
    println!(
        "Part 2: {}",
        intact_ids
            .iter()
            .next()
            .map_or_else(|| "Not found".to_string(), |id| id.to_string())
    );
}
