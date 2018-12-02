use std::io;
use std::io::BufRead;

fn combinations<F: FnMut(u8) -> ()>(containers: &[u8], container_count: u8, liters: u8, fct: &mut F) {
    if !containers.is_empty() {
        let mut next_containers = containers.to_vec();
        for (index, container) in containers.iter().enumerate().rev() {
            if *container == liters {
                fct(container_count);
            }
            else if *container < liters {
                next_containers.remove(index);
                combinations(&next_containers, container_count + 1, liters - container, fct);
            }
        }
    }
}

fn main() {
    let containers: Vec<_> = io::stdin().lock().lines()
        .filter_map(|l| l.ok())
        .filter_map(|ref line| line.parse::<u8>().ok())
        .collect();

    let liters = 150;
    let mut total_combinations = 0;
    let mut min_containers_count = containers.len() as u8;
    let mut combinations_with_min_containers_count = 0;

    combinations(&containers, 0, liters, &mut |container_count| {
        total_combinations += 1;
        if container_count == min_containers_count {
            combinations_with_min_containers_count += 1;
        }
        else if container_count < min_containers_count {
            min_containers_count = container_count;
            combinations_with_min_containers_count = 1;
        }
    });

    println!("Total combination: {}", total_combinations);
    println!("Combinations with minimum containers count: {}", combinations_with_min_containers_count);
}
