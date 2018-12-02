use std::io::stdin;
use std::io::Read;

fn map_keypad1(ch: char, state: u8) -> u8 {
    match (ch, state) {
        ('U', s) if s >= 4 => s - 3,
        ('L', s) if s % 3 != 1 => s - 1,
        ('R', s) if s % 3 != 0 => s + 1,
        ('D', s) if s <= 6 => s + 3,
        (_, s) => s,
    }
}

fn keypad1_char(state: u8) -> char {
    (state + b'0') as char
}

fn map_keypad2(ch: char, state: (i8, i8)) -> (i8, i8) {
    let raw_new_state = match (ch, state) {
        ('U', (x, y)) => (x, y + 1),
        ('L', (x, y)) => (x - 1, y),
        ('R', (x, y)) => (x + 1, y),
        ('D', (x, y)) => (x, y - 1),
        (_, s) => s,
    };
    if raw_new_state.0.abs() + raw_new_state.1.abs() <= 2 {
        raw_new_state
    } else {
        state
    }
}

fn keypad2_char(state: (i8, i8)) -> char {
    (match state {
        (i, 2) => i + ('1' as i8),
        (i, 1) => i + ('3' as i8),
        (i, 0) => i + ('7' as i8),
        (i, -1) => i + ('B' as i8),
        (i, -2) => i + ('D' as i8),
        _ => 'U' as i8,
    }) as u8 as char
}

fn main() {
    let (code_1, code_2) = stdin()
        .bytes()
        .filter_map(|b| b.ok())
        .map(|b| b as char)
        .scan((5, (-2, 0)), |state, ch| {
            Some(if ch == '\n' {
                Some((keypad1_char(state.0), keypad2_char(state.1)))
            } else {
                *state = (map_keypad1(ch, state.0), map_keypad2(ch, state.1));
                None
            })
        })
        .filter_map(|b| b)
        .fold(
            (String::new(), String::new()),
            |(mut s1, mut s2), (ch1, ch2)| {
                s1.push(ch1);
                s2.push(ch2);
                (s1, s2)
            },
        );
    println!("Part 1: {}", code_1);
    println!("Part 2: {}", code_2);
}
