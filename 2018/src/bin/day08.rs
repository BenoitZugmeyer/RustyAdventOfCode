use std::io::{stdin, BufRead};

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

fn create_tree(input: &mut impl Iterator<Item = usize>) -> Node {
    let child_nodes_count = input.next().unwrap();
    let metadata_count = input.next().unwrap();
    Node {
        children: (0..child_nodes_count).map(|_| create_tree(input)).collect(),
        metadata: input.take(metadata_count).collect(),
    }
}

fn metadata_sum(root: &Node) -> usize {
    root.metadata.iter().sum::<usize>() + root.children.iter().map(metadata_sum).sum::<usize>()
}

fn node_value(root: &Node) -> usize {
    if root.children.is_empty() {
        root.metadata.iter().sum::<usize>()
    } else {
        root.metadata
            .iter()
            .filter_map(|m| root.children.get(m.wrapping_sub(1)))
            .map(node_value)
            .sum::<usize>()
    }
}

fn main() {
    let line = stdin()
        .lock()
        .lines()
        .filter_map(|l| l.ok())
        .next()
        .unwrap_or_else(String::new);

    let root = create_tree(&mut line.split(' ').filter_map(|l| l.parse().ok()));

    println!("Part 1: {}", metadata_sum(&root));
    println!("Part 2: {}", node_value(&root));
}
