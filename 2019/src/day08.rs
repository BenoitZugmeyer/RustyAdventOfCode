use itertools::iproduct;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn layer(image: &[u8], n: usize) -> Option<&[u8]> {
    let pixels_in_layer = WIDTH * HEIGHT;
    if n * pixels_in_layer < image.len() {
        Some(&image[n * pixels_in_layer..(n + 1) * pixels_in_layer])
    } else {
        None
    }
}

#[allow(dead_code)]
fn iter_layers(image: &[u8]) -> impl Iterator<Item = &[u8]> {
    (0..)
        .map(move |n| layer(image, n))
        .take_while(|layer| !layer.is_none())
        .filter_map(|layer| layer)
}

#[allow(dead_code)]
fn format_image(image: &[u8]) -> String {
    let mut result = String::new();
    for (y, x) in iproduct!(0..HEIGHT, 0..WIDTH) {
        let pixel = iter_layers(image)
            .map(|layer| layer[x + y * WIDTH])
            .find(|px| *px != 2);
        result.push(if pixel == Some(0) { ' ' } else { '#' });
        if x == WIDTH - 1 {
            result.push('\n');
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ocr::ocr;
    use crate::util;

    fn get_image() -> Vec<u8> {
        util::input(8)
            .flat_map(|line| line.into_bytes().into_iter().map(|b| b - b'0'))
            .collect()
    }

    #[test]
    fn part_1() {
        let image = get_image();
        let layer = iter_layers(&image)
            .min_by_key(|layer| bytecount::count(layer, 0))
            .expect("Layer not found");

        assert_eq!(
            Some(bytecount::count(layer, 1) * bytecount::count(layer, 2)),
            util::answer(8, 1)
        );
    }

    #[test]
    fn part_2() {
        let image = get_image();
        let formated_image = format_image(&image);
        assert_eq!(Some(ocr(&formated_image)), util::answer(8, 2));
    }
}
