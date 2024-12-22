#![feature(iter_map_windows)]
use std::collections::{HashMap, HashSet};
advent_of_code::solution!(22);

fn mix(value: u64, secret_number: u64) -> u64 {
    value ^ secret_number
}

fn prune(value: u64) -> u64 {
    value % 16777216
}

fn first_step(secret_number: u64) -> u64 {
    prune(mix(secret_number * 64, secret_number))
}

fn second_step(secret_number: u64) -> u64 {
    prune(mix(secret_number / 32, secret_number))
}

fn third_step(secret_number: u64) -> u64 {
    prune(mix(secret_number * 2048, secret_number))
}

fn generate_next_secret_number(secret_number: u64) -> u64 {
    third_step(second_step(first_step(secret_number)))
}

fn generate_next_n_secret_numbers(secret_number: u64, n: usize) -> Vec<u64> {
    let mut secret_numbers = Vec::with_capacity(n + 1);
    secret_numbers.push(secret_number);
    let mut current_secret_number = secret_number;
    for _ in 0..n {
        let next_secret_number = generate_next_secret_number(current_secret_number);
        secret_numbers.push(next_secret_number);
        current_secret_number = next_secret_number;
    }
    secret_numbers
}

fn update_sequence_map(
    secret_number: u64,
    n: usize,
    sequence_to_bananas: &mut HashMap<[i64; 4], u64>,
) {
    let mut bought: HashSet<[i64; 4]> = HashSet::new();
    for window in generate_next_n_secret_numbers(secret_number, n)
        .iter()
        .map(|secret_number| secret_number % 10)
        .map_windows(|[a, b]| (*b as i64 - *a as i64, *b))
        .collect::<Vec<(i64, u64)>>()
        .windows(4)
    {
        let price = window[3].1;
        let window = [window[0].0, window[1].0, window[2].0, window[3].0];
        if bought.insert(window) {
            sequence_to_bananas
                .entry(window)
                .and_modify(|number_of_bananas| *number_of_bananas += price)
                .or_insert(price);
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| generate_next_n_secret_numbers(line.parse::<u64>().unwrap(), 2000)[2000])
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sequence_to_bananas = HashMap::new();
    input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .for_each(|secret_number| {
            update_sequence_map(secret_number, 2000, &mut sequence_to_bananas)
        });
    let (_, &result) = sequence_to_bananas
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .unwrap();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(23));
    }
}
