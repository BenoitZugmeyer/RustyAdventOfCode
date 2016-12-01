#![feature(io)]
use std::io::stdin;
use std::io::Read;

fn main() {

    let (negative_floor, final_floor) = stdin().chars()
        .scan(0, |acc, result| {
            match result {
                Ok('(') => *acc += 1,
                Ok(')') => *acc -= 1,
                _ => {},
            };
            Some(*acc)
        })
        .enumerate()
        .fold((None, 0), |(negative_floor, _), (index, floor)| {
            (
                if negative_floor.is_none() && floor == -1 { Some(index + 1) } else { negative_floor },
                floor,
            )
        });

    println!("Final floor: {}", final_floor);

    if let Some(floor) = negative_floor {
        println!("Position of the floor -1: {}", floor);
    } else {
        println!("Santa doesn't go to the floor -1");
    }

}
