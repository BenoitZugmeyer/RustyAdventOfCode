#[cfg(test)]
mod tests {
    use crate::intcode::{Program, Value};
    use crate::util;
    use itertools::Itertools;
    use std::collections::VecDeque;
    use std::convert::TryFrom;

    type Computer = (Program, VecDeque<(Value, Value)>);

    fn get_program() -> Program {
        util::input(23).next().expect("No input").parse().unwrap()
    }

    fn get_computers() -> Vec<Computer> {
        let program = get_program();
        (0..50)
            .map(|address| {
                let mut program = program.clone();
                program.run(&[Value::from(address)]);
                (program, VecDeque::new())
            })
            .collect()
    }

    fn run_computer((program, queue): &mut Computer) -> Vec<(usize, (Value, Value))> {
        (if let Some((x, y)) = queue.pop_front() {
            program.run(&[x, y])
        } else {
            program.run(&[-1])
        })
        .get_output()
        .into_iter()
        .tuples()
        .map(|(address, x, y)| (usize::try_from(*address).unwrap(), (*x, *y)))
        .collect()
    }

    #[test]
    fn part_1() {
        let mut computers = get_computers();

        loop {
            for address in 0..computers.len() {
                let output = run_computer(&mut computers[address]);

                for (destination, packet) in output {
                    if destination == 255 {
                        assert_eq!(Some(packet.1), util::answer(23, 1));
                        return;
                    }
                    computers[destination].1.push_back(packet);
                }
            }
        }
    }

    #[test]
    fn part_2() {
        let mut computers = get_computers();

        let mut nat = (0, 0);
        let mut previously_delivered = 0;

        loop {
            for address in 0..computers.len() {
                let output = run_computer(&mut computers[address]);

                for (destination, packet) in output {
                    if destination == 255 {
                        nat = packet;
                    } else {
                        computers[destination].1.push_back(packet);
                    }
                }
            }

            if computers.iter().all(|(_, queue)| queue.is_empty()) {
                if previously_delivered == nat.1 {
                    assert_eq!(Some(previously_delivered), util::answer(23, 2));
                    return;
                }
                computers[0].1.push_back(nat);
                previously_delivered = nat.1;
            }
        }
    }
}
