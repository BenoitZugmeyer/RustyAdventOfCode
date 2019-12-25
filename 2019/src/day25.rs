#[cfg(test)]
mod tests {
    use crate::intcode::{Program, Value};
    use crate::util;
    use itertools::Itertools;
    use std::collections::HashSet;

    fn get_program() -> Program {
        util::input(25).next().expect("No input").parse().unwrap()
    }

    #[test]
    fn part_1() {
        let mut program = get_program();

        let inputs = [
            "",
            // Hull Breach
            "south\n",
            // Stables
            "take fixed point\n",
            "north\n",
            // Hull breach
            "north\n",
            // Corridor
            "take spool of cat6\n",
            "north\n",
            // Observatory
            "take monolith\n",
            "west\n",
            // Holodeck
            "take planetoid\n",
            "east\n",
            // Observatory
            "north\n",
            // Science lab
            "take hypercube\n",
            "south\n",
            // Observatory
            "south\n",
            // Corridor
            "east\n",
            // Gift wrapping center
            "north\n",
            // Hallway
            "take candy cane\n",
            "west\n",
            // Hot chocolate fountain
            // don't "take photons\n",
            "east\n",
            // Hallway
            "east\n",
            // Kitchen
            // don't "take giant electromagnet\n",
            "west\n",
            // Hallway
            "south\n",
            // Gift wrapping center
            "east\n",
            // Arcade
            "take easter egg\n",
            "east\n",
            // Passages
            // don't "take escape pod\n",
            "east\n",
            // Navigation
            // don't "take molten lava\n",
            "west\n",
            // Passages
            "south\n",
            // Engineering
            "take ornament\n",
            "east\n",
            // Warp drive maintenance
            // don't "take infinite loop\n",
            "west\n",
            // Engineering
            "west\n",
            // Sick Bay
            "south\n",
            // Security checkpoint
            "inv\n",
            "drop planetoid\n",
            "drop spool of cat6\n",
            "drop candy cane\n",
            "drop fixed point\n",
            "west\n",
        ];
        for input in inputs.iter() {
            println!(">>> {}", input);
            program.run_str(&input).print_ascii();
        }

        let items: HashSet<&str> = [
            "drop planetoid\n",
            "drop candy cane\n",
            "drop ornament\n",
            "drop easter egg\n",
            "drop spool of cat6\n",
            "drop fixed point\n",
            "drop hypercube\n",
            "drop monolith\n",
        ]
        .into_iter()
        .cloned()
        .collect();

        for len in 0..items.len() {
            for items in items.iter().combinations(len) {
                let mut lprogram = program.clone();
                for item in &items {
                    lprogram.run_str(item);
                }
                let result = lprogram.run_str("west\n");
                if !search_in_output(&result.get_output(), b" heavier ")
                    && !search_in_output(&result.get_output(), b" lighter ")
                {
                    dbg!(&items);
                    result.print_ascii();
                }
            }
        }
    }

    fn search_in_output(output: &[Value], needle: &[u8]) -> bool {
        let needle = needle.iter().cloned().map(Value::from).collect::<Vec<_>>();
        output.windows(needle.len()).any(|w| w == needle.as_slice())
    }
}
