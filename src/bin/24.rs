use std::collections::HashMap;

use Gate::*;
use Signal::*;
advent_of_code::solution!(24);

type Input<'a> = (&'a Signal, &'a Signal);
type Output<'a> = &'a Signal;

#[derive(Debug)]
enum Signal {
    On,
    Off,
    NoValue,
}

#[derive(Debug)]
enum Gate<'a> {
    AND(Input<'a>, Output<'a>),
    OR(Input<'a>, Output<'a>),
    XOR(Input<'a>, Output<'a>),
}

fn parse_input(input: &str) -> HashMap<String, Signal> {
    let mut wires: HashMap<String, Signal> = HashMap::new();
    input
        .lines()
        .filter(|line| line.contains(": "))
        .map(|line| line.split(": ").collect::<Vec<&str>>())
        .for_each(|wire| {
            wires.insert(
                wire[0].to_owned(),
                if wire[1].contains("1") { On } else { Off },
            );
        });
    let mut gates: Vec<Gate> = vec![];
    for gate in input
        .lines()
        .filter(|line| line.contains("->"))
        .map(|line| line.split("->").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>()
    {
        // insert rightside
        wires.insert(gate[1].to_owned(), NoValue);
        // insert leftside
        let leftside = gate[0].split(" ").collect::<Vec<&str>>();
        let (op1, op2) = (leftside[0], leftside[2]);
        if !wires.contains_key(op1) {
            wires.insert(op1.to_owned(), NoValue);
        }
        if !wires.contains_key(op2) {
            wires.insert(op2.to_owned(), NoValue);
        }
        let input: Input = (&wires[op1], &wires[op2]);
        let output: Output = &wires[gate[1]];
        let gate = match leftside[1] {
            "AND" => AND(input, output),
            "OR" => OR(input, output),
            "XOR" => XOR(input, output),
            _ => unreachable!("Wrong gate"),
        };
        // println!("Gate: {gate:?}");
        gates.push(gate);
    }
    wires
}

pub fn part_one(input: &str) -> Option<u32> {
    let wires = parse_input(input);
    println!("{wires:?}");
    None
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
