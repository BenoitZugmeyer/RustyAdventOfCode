use std::io;
use std::io::BufRead;

struct Divisors {
    n: u32,
    max: u32,
    index: u32,
}

impl Divisors {
    fn new(n: u32) -> Self {
        let max = f64::from(n).sqrt() as u32;
        Divisors { n, max, index: 0 }
    }
}

impl Iterator for Divisors {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.max {
            self.index += 1;

            if self.n % self.index == 0 {
                return Some((self.index, self.n / self.index));
            }
        }
        None
    }
}

struct HouseIterator {
    index: u32,
    max_houses: u32,
    present_factor: u32,
}

impl HouseIterator {
    fn new(present_factor: u32, max_houses: u32) -> Self {
        HouseIterator {
            index: 0,
            present_factor,
            max_houses,
        }
    }

    fn min_house(&mut self, min_presents: u32) -> Option<u32> {
        self.skip_while(|&(_, presents)| presents < min_presents)
            .next()
            .map(|(house, _)| house)
    }
}

impl Iterator for HouseIterator {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        let mut presents = 0;
        for (a, b) in Divisors::new(self.index) {
            if a > self.max_houses {
                break;
            }
            if a == b {
                presents += a;
            } else {
                presents += a + b;
            }
        }
        Some((self.index, presents * self.present_factor))
    }
}

fn main() {
    let min_presents = io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let mut infinite_delivery = HouseIterator::new(10, u32::max_value());
    if let Some(house) = infinite_delivery.min_house(min_presents) {
        println!("Infinite delivery min house: {}", house);
    }

    let mut maxed_delivery = HouseIterator::new(11, 50);
    if let Some(house) = maxed_delivery.min_house(min_presents) {
        println!("Maxed delivery min house: {}", house);
    }
}
