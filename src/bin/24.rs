use std::collections::HashMap;

use Gate::*;
use Signal::*;
advent_of_code::solution!(24);

type Input = (usize, usize);
type Output = usize;

#[derive(Debug, PartialEq, Clone)]
enum Signal {
    On,
    Off,
    NoValue,
}

#[derive(Debug)]
enum Gate {
    AND(Input, Output),
    OR(Input, Output),
    XOR(Input, Output),
}

impl Gate {
    fn calculate(&self, wires: &mut Vec<Signal>) {
        if !self.need_input(wires) {
            match *self {
                AND((op1, op2), output) => {
                    if wires[op1] == On && wires[op2] == On {
                        wires[output] = On
                    } else {
                        wires[output] = Off
                    }
                }
                OR((op1, op2), output) => {
                    if wires[op1] == On || wires[op2] == On {
                        wires[output] = On
                    } else {
                        wires[output] = Off
                    }
                }
                XOR((op1, op2), output) => {
                    if wires[op1] != wires[op2] {
                        wires[output] = On
                    } else {
                        wires[output] = Off
                    }
                }
            }
        }
    }

    fn is_done(&self, wires: &Vec<Signal>) -> bool {
        match *self {
            AND(_, output) => wires[output] != NoValue,
            OR(_, output) => wires[output] != NoValue,
            XOR(_, output) => wires[output] != NoValue,
        }
    }

    fn need_input(&self, wires: &Vec<Signal>) -> bool {
        match *self {
            AND((op1, op2), _) => wires[op1] == NoValue || wires[op2] == NoValue,
            OR((op1, op2), _) => wires[op1] == NoValue || wires[op2] == NoValue,
            XOR((op1, op2), _) => wires[op1] == NoValue || wires[op2] == NoValue,
        }
    }
}

fn parse_input(input: &str) -> (Vec<Signal>, HashMap<String, usize>, Vec<Gate>) {
    let mut wires: Vec<Signal> = vec![];
    let mut wires_indices: HashMap<String, usize> = HashMap::new();
    input
        .lines()
        .filter(|line| line.contains(": "))
        .map(|line| line.split(": ").collect::<Vec<&str>>())
        .for_each(|wire| {
            wires.push(if wire[1].contains("1") { On } else { Off });
            wires_indices.insert(wire[0].to_owned(), wires.len() - 1);
        });
    let mut gates: Vec<Gate> = vec![];
    for gate in input
        .lines()
        .filter(|line| line.contains("->"))
        .map(|line| {
            line.split("->")
                .map(|side| side.trim())
                .collect::<Vec<&str>>()
        })
        .collect::<Vec<Vec<&str>>>()
    {
        // insert rightside
        if !wires_indices.contains_key(gate[1]) {
            wires.push(NoValue);
            wires_indices.insert(gate[1].to_owned(), wires.len() - 1);
        }
        // insert leftside
        let leftside = gate[0]
            .split(" ")
            .map(|part| part.trim())
            .collect::<Vec<&str>>();
        let (op1, op2) = (leftside[0], leftside[2]);
        if !wires_indices.contains_key(op1) {
            wires.push(NoValue);
            wires_indices.insert(op1.to_owned(), wires.len() - 1);
        }
        if !wires_indices.contains_key(op2) {
            wires.push(NoValue);
            wires_indices.insert(op2.to_owned(), wires.len() - 1);
        }
        let input: Input = (wires_indices[op1], wires_indices[op2]);
        let output: Output = wires_indices[gate[1]];
        let gate = match leftside[1] {
            "AND" => AND(input, output),
            "OR" => OR(input, output),
            "XOR" => XOR(input, output),
            _ => unreachable!("Wrong gate"),
        };
        gates.push(gate);
    }
    (wires, wires_indices, gates)
}

fn _print_gates(wires: &Vec<Signal>, gates: &Vec<Gate>) {
    for gate in gates {
        let (op1, op2, output, gate_type): (usize, usize, usize, &str);
        match gate {
            AND((in1, in2), out) => (op1, op2, output, gate_type) = (*in1, *in2, *out, "AND"),
            OR((in1, in2), out) => (op1, op2, output, gate_type) = (*in1, *in2, *out, "OR"),
            XOR((in1, in2), out) => (op1, op2, output, gate_type) = (*in1, *in2, *out, "XOR"),
        }
        println!(
            "{:?} {gate_type} {:?} -> {:?}",
            wires[op1], wires[op2], wires[output]
        )
    }
}

fn _print_wire_values(wires: &Vec<Signal>, wire_indices: &HashMap<String, usize>) {
    let mut keys = wire_indices
        .keys()
        .map(|key| key.to_owned())
        .collect::<Vec<String>>();
    keys.sort();
    for key in keys
        .iter()
        .filter(|key| !key.contains('x') && !key.contains('y'))
    {
        println!("{key}: {:?}", wires[wire_indices[key]])
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut wires, wire_indices, gates) = parse_input(input);
    let mut done = gates.iter().filter(|gate| gate.is_done(&wires)).count();
    while !gates.iter().all(|gate| gate.is_done(&wires)) {
        for gate in gates.iter() {
            gate.calculate(&mut wires);
        }
        let newly_done = gates.iter().filter(|gate| gate.is_done(&wires)).count() - done;

        if newly_done == 0 {
            break;
        } else {
            done += newly_done
        }
    }

    let mut sum: u64 = 0;
    for i in 0..64 {
        let wire_name = format!("z{:02}", i);
        if wire_indices.contains_key(&wire_name) {
            let result = wires[wire_indices[&wire_name]].clone();
            if result == On {
                sum += 2_u64.pow(i)
            }
        } else {
            break;
        }
    }
    _print_wire_values(&wires, &wire_indices);

    Some(sum)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_small() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_big() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, None);
    }
}
