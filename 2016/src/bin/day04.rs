extern crate itertools;
use itertools::Itertools;
use std::collections::HashMap;
use std::mem;
use std::io::stdin;
use std::io::Read;
use std::cmp::Ordering;

fn main() {

    let (sector_sum, room) = stdin()
        .bytes()
        .filter_map(|b| b.ok())
        .map(|b| b as char)
        .scan((Vec::new(), Vec::new(), 0, false), |state, ch| {
            Some(match ch {
                '\n' => {
                    let previous_state: (Vec<char>, Vec<char>, u32, bool) =
                        mem::replace(state, (Vec::new(), Vec::new(), 0, false));
                    Some((previous_state.0, previous_state.1, previous_state.2))
                }
                '[' => {
                    state.3 = true;
                    None
                }
                c @ '0'...'9' => {
                    state.2 = state.2 * 10 + c as u32 - '0' as u32;
                    None
                }
                'a'...'z' | '-' => {
                    if state.3 {
                        state.1.push(ch);
                    } else {
                        state.0.push(ch);
                    }
                    None
                }
                _ => None,
            })
        })
        .filter_map(|b| b)
        .filter(|&(ref name, ref hash, _)| {
            let mut map = HashMap::new();
            for ch in name {
                if ch != &'-' {
                    *map.entry(ch).or_insert(0) += 1;
                }
            }

            let expected_hash: Vec<_> = map.iter()
                    .sorted_by(|&(ch_a, count_a), &(ch_b, count_b)| {
                        match Ord::cmp(count_a, count_b).reverse() {
                            Ordering::Equal => Ord::cmp(ch_a, ch_b),
                            ord => ord,
                        }
                    })[..5]
                .into_iter()
                .map(|&(ch, _)| **ch)
                .collect();

            &expected_hash == hash
        })
        .fold((0, None), |(sector_sum, room), (name, _, sector)| {
            let new_room = room.or_else(|| {
                let mut decrypted_name = String::new();
                let base = 'a' as u32;
                for ch in &name[..name.len() - 1] {
                    if ch != &'-' {
                        decrypted_name.push(
                            (((*ch as u32 - base) + sector) % 26 + base) as u8 as char
                        );
                    } else {
                        decrypted_name.push(' ');
                    }
                }
                if decrypted_name == "northpole object storage" { Some(sector) } else { None }
            });
            (sector_sum + sector, new_room)
        });

    println!("Part 1: {}", sector_sum);
    println!("Part 2: {}", room.expect("No room found"));
}
