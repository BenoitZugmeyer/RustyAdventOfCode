#[macro_use]
extern crate nom;
extern crate itertools;

use std::io::stdin;
use std::io::Read;

use itertools::Itertools;

#[derive(Debug, Ord, Eq, PartialOrd, PartialEq, Clone, Hash)]
enum Instruction {
    SwapPosition(usize, usize),
    SwapLetter(char, char),
    Rotate(i8),
    RotateBasedOnLetter(char),
    Reverse(usize, usize),
    Move(usize, usize),
}

fn rotate(s: &mut Vec<char>, x: i8) {
    let count = x % s.len() as i8;
    if count != 0 {
        *s = {
            let (before, after) = s.split_at(if count > 0 {
                s.len() as i8 - count
            } else {
                -count
            } as usize);
            let mut result = Vec::with_capacity(s.len());
            result.extend(after);
            result.extend(before);
            result
        };
    };
}

impl Instruction {
    fn apply(&self, mut s: &mut Vec<char>) {
        match *self {
            Instruction::SwapPosition(x, y) => s.swap(x, y),
            Instruction::SwapLetter(x, y) => {
                let px = s.iter().position(|c| *c == x).unwrap();
                let py = s.iter().position(|c| *c == y).unwrap();
                s.swap(px, py);
            }
            Instruction::Rotate(x) => rotate(&mut s, x),
            Instruction::RotateBasedOnLetter(x) => {
                let mut px = s.iter().position(|c| *c == x).unwrap();
                if px >= 4 {
                    px += 1;
                }
                rotate(&mut s, px as i8 + 1);
            }
            Instruction::Reverse(x, y) => {
                for i in 0..(y - x + 1) / 2 {
                    s.swap(x + i, y - i);
                }
            }
            Instruction::Move(x, y) => {
                if x < y {
                    for i in x..y {
                        s.swap(i, i + 1);
                    }
                } else {
                    for i in (y..x).rev() {
                        s.swap(i, i + 1);
                    }
                }
            }
        };
    }

    fn unapply(&self, mut s: &mut Vec<char>) {
        match *self {
            Instruction::SwapPosition(x, y) => Instruction::SwapPosition(y, x).apply(&mut s),
            Instruction::SwapLetter(x, y) => Instruction::SwapLetter(y, x).apply(&mut s),
            Instruction::Rotate(x) => Instruction::Rotate(-x).apply(&mut s),
            Instruction::RotateBasedOnLetter(x) => {
                // i -> d -> ni
                // 0 -> 1 -> 1 > 1
                // 1 -> 2 -> 3 > 3
                // 2 -> 3 -> 5 > 5
                // 3 -> 4 -> 7 > 7
                // 4 -> 6 -> 10 > 2
                // 5 -> 7 -> 12 > 4
                // 6 -> 8 -> 14 > 6
                // 7 -> 9 -> 16 > 0
                let px = s.iter().position(|c| *c == x).unwrap();
                let previous_pos = if px % 2 == 1 {
                    (px - 1) / 2
                } else if px > 0 {
                    3 + px / 2
                } else {
                    7
                };

                rotate(&mut s, previous_pos as i8 - px as i8);
            }
            Instruction::Reverse(x, y) => Instruction::Reverse(x, y).apply(&mut s),
            Instruction::Move(x, y) => Instruction::Move(y, x).apply(&mut s),
        };
    }
}

#[test]
fn test_swap_position() {
    let mut a = vec!['a', 'b', 'c', 'd'];
    Instruction::SwapPosition(1, 2).apply(&mut a);
    assert_eq!(a, vec!['a', 'c', 'b', 'd']);

    let mut b = vec!['a', 'b', 'c', 'd', 'e'];
    Instruction::SwapPosition(1, 3).apply(&mut b);
    assert_eq!(b, vec!['a', 'd', 'c', 'b', 'e']);
}

#[test]
fn test_swap_letter() {
    let mut a = vec!['a', 'b', 'c', 'd'];
    Instruction::SwapLetter('b', 'c').apply(&mut a);
    assert_eq!(a, vec!['a', 'c', 'b', 'd']);

    let mut b = vec!['a', 'b', 'c', 'd', 'e'];
    Instruction::SwapLetter('b', 'd').apply(&mut b);
    assert_eq!(b, vec!['a', 'd', 'c', 'b', 'e']);
}

#[test]
fn test_rotate() {
    let mut a = vec!['a', 'b', 'c', 'd'];
    Instruction::Rotate(1).apply(&mut a);
    assert_eq!(a, vec!['d', 'a', 'b', 'c']);

    Instruction::Rotate(-1).apply(&mut a);
    assert_eq!(a, vec!['a', 'b', 'c', 'd']);
}

#[test]
fn test_rotate_based_on_letter() {
    let mut a = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    Instruction::RotateBasedOnLetter('b').apply(&mut a); // 2 times
    assert_eq!(a, vec!['f', 'g', 'a', 'b', 'c', 'd', 'e']);

    Instruction::RotateBasedOnLetter('c').apply(&mut a); // 6 times
    assert_eq!(a, vec!['g', 'a', 'b', 'c', 'd', 'e', 'f']);
}

#[test]
fn test_reverse() {
    let mut a = vec!['a', 'b', 'c', 'd', 'e'];
    Instruction::Reverse(1, 3).apply(&mut a);
    assert_eq!(a, vec!['a', 'd', 'c', 'b', 'e']);

    let mut b = vec!['a', 'b', 'c', 'd', 'e'];
    Instruction::Reverse(1, 2).apply(&mut b);
    assert_eq!(b, vec!['a', 'c', 'b', 'd', 'e']);
}

#[test]
fn test_move() {
    let mut a = vec!['a', 'b', 'c', 'd', 'e'];
    Instruction::Move(1, 3).apply(&mut a);
    assert_eq!(a, vec!['a', 'c', 'd', 'b', 'e']);

    let mut b = vec!['a', 'b', 'c', 'd', 'e'];
    Instruction::Move(3, 1).apply(&mut b);
    assert_eq!(b, vec!['a', 'd', 'b', 'c', 'e']);
}

named!(
    parse_number<i32>,
    do_parse!(
        minus: opt!(tag!("-")) >>
        n: take_while1!(nom::is_digit) >>
        (if minus == Some(b"-") { -1 } else { 1 } *
         n.iter().fold(0, |acc, item| acc * 10 + (item - b'0') as i32))
    )
);

named!(
    parse_char<char>,
    do_parse!(
        ch: take!(1) >>
        (ch[0] as char)
    )
);

named!(
    parse_instruction(&[u8]) -> Instruction,
    alt!(
        do_parse!(
            tag!("swap position ") >>
            x: parse_number >>
            tag!(" with position ") >>
            y: parse_number >>
            (Instruction::SwapPosition(x as usize, y as usize))
        )
        |
        do_parse!(
            tag!("swap letter ") >>
            x: parse_char >>
            tag!(" with letter ") >>
            y: parse_char >>
            (Instruction::SwapLetter(x, y))
        )
        |
        do_parse!(
            tag!("rotate left ") >>
            x: parse_number >>
            tag!(" step") >>
            (Instruction::Rotate(-x as i8))
        )
        |
        do_parse!(
            tag!("rotate right ") >>
            x: parse_number >>
            tag!(" step") >>
            (Instruction::Rotate(x as i8))
        )
        |
        do_parse!(
            tag!("rotate based on position of letter ") >>
            x: parse_char >>
            (Instruction::RotateBasedOnLetter(x))
        )
        |
        do_parse!(
            tag!("reverse positions ") >>
            x: parse_number >>
            tag!(" through ") >>
            y: parse_number >>
            (Instruction::Reverse(x as usize, y as usize))
        )
        |
        do_parse!(
            tag!("move position ") >>
            x: parse_number >>
            tag!(" to position ") >>
            y: parse_number >>
            (Instruction::Move(x as usize, y as usize))
        )
    )
);

fn main() {

    let mut input: Vec<_> = "abcdefgh".chars().collect();

    let instructions: Vec<_> = stdin()
        .bytes()
        .filter_map(|b| b.ok())
        .batching(|it| {
            let bytes: Vec<_> = it.take_while(|ch| ch != &b'\n').collect();
            if bytes.is_empty() {
                None
            } else {
                Some(match parse_instruction(bytes.as_slice()) {
                    nom::IResult::Error(e) => {
                        panic!(
                            "Failed to parse '{}': {}",
                            String::from_utf8_lossy(&bytes),
                            e
                        );
                    }
                    nom::IResult::Incomplete(_) => {
                        panic!(
                            "Failed to parse '{}': incomplete input",
                            String::from_utf8_lossy(&bytes),
                        );
                    }
                    nom::IResult::Done(_, o) => o,
                })
            }
        })
        .collect();

    for instruction in &instructions {
        instruction.apply(&mut input);
    }

    println!("Part 1: {}", input.iter().cloned().collect::<String>());

    let mut input2: Vec<_> = "fbgdceah".chars().collect();
    for instruction in instructions.iter().rev() {
        instruction.unapply(&mut input2);
    }

    println!("Part 2: {}", input2.iter().cloned().collect::<String>());
}
