extern crate regex;
use regex::Regex;
use std::io::stdin;
use std::io::BufRead;
use std::collections::{HashSet, HashMap};

#[derive(Debug)]
struct ParsedTree {
    name: String,
    weight: u32,
    children: Vec<String>,
}

fn get_unbalanced(trees: &HashMap<String, ParsedTree>, tree: &ParsedTree) -> (u32, bool) {
    let mut weights: Vec<u32> = Vec::new();
    for child in &tree.children {
        let (child_weight, unbalanced) = get_unbalanced(trees, &trees[child]);
        if unbalanced {
            return (child_weight, true);
        }
        weights.push(child_weight);
    }
    if weights.len() >= 3 {
        let expected_weight = if weights[0] == weights[1] || weights[0] == weights[2] {
            weights[0]
        } else {
            weights[1]
        };
        if let Some(unbalanced_position) = weights.iter().position(|w| *w != expected_weight) {
            let unbalanced = &trees[&tree.children[unbalanced_position]];
            return (
                unbalanced.weight + expected_weight - weights[unbalanced_position],
                true,
            );
        }
    }
    (weights.iter().sum::<u32>() + tree.weight, false)
}

fn main() {
    let stdin = stdin();
    let re = Regex::new(r"\w+").expect("failed to parse regex");
    let trees: HashMap<_, _> = stdin
        .lock()
        .lines()
        .filter_map(|l| l.ok())
        .map(|line| {
            let captures: Vec<_> = re.captures_iter(&line).collect();
            let name = captures[0]
                .get(0)
                .expect("failed to unwrap name capture")
                .as_str()
                .to_string();
            (
                name.clone(),
                ParsedTree {
                    name,
                    weight: captures[1]
                        .get(0)
                        .expect("failed to unwrap weight capture")
                        .as_str()
                        .parse()
                        .expect("failed to parse weight"),
                    children: captures[2..]
                        .iter()
                        .map(|cap| {
                            cap.get(0)
                                .expect("failed to unwrap child capture")
                                .as_str()
                                .to_string()
                        })
                        .collect(),
                },
            )
        })
        .collect();

    let non_roots: HashSet<_> = trees
        .values()
        .flat_map(|tree| tree.children.iter())
        .collect();
    let root = trees
        .values()
        .find(|tree| !non_roots.contains(&tree.name))
        .unwrap();

    println!("Part 1: {}", root.name);
    println!("Part 2: {}", get_unbalanced(&trees, root).0);
}
