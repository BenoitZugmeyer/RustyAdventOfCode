use lazy_static::lazy_static;
use std::collections::HashMap;

const CHAR_WIDTH: usize = 4;
lazy_static! {
    static ref CHARS: HashMap<&'static str, char> = {
        [
            (" ## #  ##  ######  ##  #", 'A'),
            ("### #  #### #  ##  #### ", 'B'),
            ("#####   ### #   #   ####", 'E'),
            ("#####   ### #   #   #   ", 'F'),
            (" ## #  ##   # ###  # ###", 'G'),
            ("  ##   #   #   ##  # ## ", 'J'),
            ("#   #   #   #   #   ####", 'L'),
            ("### #  ##  #### #   #   ", 'P'),
            ("### #  ##  #### # # #  #", 'R'),
            ("#   #    # #  #   #   # ", 'Y'),
            ("####   #  #  #  #   ####", 'Z'),
        ]
        .iter()
        .cloned()
        .collect()
    };
}

#[allow(dead_code)]
pub fn ocr(image: &str) -> String {
    let image: Vec<_> = image.split('\n').filter(|line| !line.is_empty()).collect();

    let height = image.len();
    let width = image[0].len();
    (0..width)
        .step_by(CHAR_WIDTH + 1)
        .map(|i| {
            let image_char: String = (0..height).map(|y| &image[y][i..i + CHAR_WIDTH]).collect();
            if let Some(ch) = CHARS.get(image_char.as_str()) {
                ch
            } else {
                panic!("Character not found: {:?}", image_char);
            }
        })
        .collect()
}
