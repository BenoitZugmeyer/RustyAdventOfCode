use std::io;
use std::io::BufRead;

fn is_password_valid(password: &[u8]) -> bool {
    let mut previous = None;
    let mut char_pair_count = 0;
    let mut has_three_increasing_chars = false;

    for (index, &ch) in password.iter().enumerate() {
        match ch {
            b'i' | b'l' | b'o' => return false,
            _ => {},
        }

        if previous == Some(ch) {
            char_pair_count += 1;
            previous = None;
        }
        else {
            previous = Some(ch);
        }

        if ! has_three_increasing_chars {
            has_three_increasing_chars =
                index + 2 < password.len() &&
                password[index + 1] == ch + 1 &&
                password[index + 2] == ch + 2;
        }
    }

    char_pair_count >= 2 && has_three_increasing_chars
}

fn next_password(password: &mut Vec<u8>) {
    let mut rem = true;
    for index in (0 .. password.len()).rev() {
        if rem {
            match password[index] {
                n if n >= b'z' => password[index] = b'a',
                _ => {
                    password[index] += 1;
                    rem = false;
                    break;
                }
            }
        }
    }

    if rem {
        password.insert(0, b'a');
    }
}

struct ValidPasswords {
    current_password: Vec<u8>,
}

impl ValidPasswords {
    fn new(password: String) -> Self {
        ValidPasswords { current_password: password.into_bytes() }
    }
}

impl Iterator for ValidPasswords {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        loop {
            next_password(&mut self.current_password);
            if is_password_valid(&self.current_password) {
                break;
            }
        }
        Some(String::from_utf8(self.current_password.clone()).unwrap())
    }
}

#[test]
fn is_password_valid_test() {
    assert_eq!(is_password_valid("hijklmmn".as_bytes()), false);
    assert_eq!(is_password_valid("abbceffg".as_bytes()), false);
    assert_eq!(is_password_valid("abbcegjk".as_bytes()), false);
    assert_eq!(is_password_valid("aabcc".as_bytes()), true);
    assert_eq!(is_password_valid("aaabc".as_bytes()), false);
}

#[test]
fn next_password_test() {
    let mut password = "abcde".to_string().into_bytes();
    next_password(&mut password);
    assert_eq!(password, [97, 98, 99, 100, 102]);

    let mut password = "abcdz".to_string().into_bytes();
    next_password(&mut password);
    assert_eq!(password, [97, 98, 99, 101, 97]);

    let mut password = "zzzz".to_string().into_bytes();
    next_password(&mut password);
    assert_eq!(password, [97, 97, 97, 97, 97]);
}

fn main() {

    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str).unwrap();
    let mut valid_passwords = ValidPasswords::new(input_str);

    println!("Next password: {}", valid_passwords.next().unwrap());
    println!("Next password: {}", valid_passwords.next().unwrap());
}
