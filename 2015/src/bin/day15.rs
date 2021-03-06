extern crate regex;

use regex::Regex;
use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct Ingredient(i32, i32, i32, i32, i32);

#[derive(Debug)]
struct Recipe {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Recipe {
    fn new() -> Self {
        Recipe {
            capacity: 0,
            durability: 0,
            flavor: 0,
            texture: 0,
            calories: 0,
        }
    }

    fn add_ingredient(&mut self, ingredient: &Ingredient, spoons: i32) {
        let &Ingredient(capacity, durability, flavor, texture, calories) = ingredient;
        self.capacity += capacity * spoons;
        self.durability += durability * spoons;
        self.flavor += flavor * spoons;
        self.texture += texture * spoons;
        self.calories += calories * spoons;
    }

    fn score(&self) -> u32 {
        if self.capacity > 0 && self.durability > 0 && self.flavor > 0 && self.texture > 0 {
            (self.capacity * self.durability * self.flavor * self.texture) as u32
        } else {
            0
        }
    }
}

impl Default for Recipe {
    fn default() -> Self {
        Self::new()
    }
}

struct Generator {
    spoons: Vec<u8>,
    max: u8,
    first: bool,
}

impl Generator {
    fn new(count: usize, max: u8) -> Self {
        let mut spoons = vec![0; count];
        spoons[0] = max;
        Generator {
            spoons,
            max,
            first: true,
        }
    }

    fn gen_next(&mut self) -> bool {
        let mut next_rem_total = self.spoons.iter().skip(1).sum::<u8>() + 1;
        let mut result = false;

        for spoon_count in self.spoons.iter_mut().skip(1) {
            if next_rem_total <= self.max {
                *spoon_count += 1;
                result = true;
                break;
            } else {
                next_rem_total -= *spoon_count;
                *spoon_count = 0;
            }
        }

        self.spoons[0] = self.max - next_rem_total;

        result
    }

    fn generate(&mut self) -> Option<&Vec<u8>> {
        if self.first || self.gen_next() {
            self.first = false;
            Some(&self.spoons)
        } else {
            None
        }
    }
}

fn main() {
    let re = Regex::new(r"^\w+: capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)").unwrap();

    let parameters: Vec<_> = io::stdin()
        .lock()
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|ref line| {
            re.captures(line).map(|ref m| {
                Ingredient(
                    m.at(1).unwrap().parse::<i32>().unwrap(),
                    m.at(2).unwrap().parse::<i32>().unwrap(),
                    m.at(3).unwrap().parse::<i32>().unwrap(),
                    m.at(4).unwrap().parse::<i32>().unwrap(),
                    m.at(5).unwrap().parse::<i32>().unwrap(),
                )
            })
        })
        .collect();

    let mut generator = Generator::new(parameters.len(), 100);
    let mut max_score = 0;
    let mut max_score_500_calories = 0;

    while let Some(spoons) = generator.generate() {
        let mut recipe = Recipe::new();
        for (ingredient, spoon_count) in parameters.iter().zip(spoons.iter()) {
            recipe.add_ingredient(ingredient, i32::from(*spoon_count));
        }

        let score = recipe.score();
        if score > max_score {
            max_score = score;
        }
        if recipe.calories == 500 && score > max_score_500_calories {
            max_score_500_calories = score;
        }
    }

    println!("Highest score: {}", max_score);
    println!(
        "Highest score for 500 calories recipes: {}",
        max_score_500_calories
    );
}
