use std::collections::{hash_map::Entry, HashMap};

advent_of_code::solution!(19);

fn is_possible(design: &Vec<char>, available_patterns: &HashMap<char, Vec<Vec<char>>>) -> bool {
    if design.len() == 0 {
        true
    } else {
        let first_color = design[0];
        available_patterns
            .get(&first_color)
            .unwrap_or(&vec![])
            .iter()
            .filter(|&pattern| {
                pattern
                    .iter()
                    .zip(design)
                    .map(|pair| *pair.0 == *pair.1)
                    .all(|can_continue| can_continue)
            })
            .any(|pattern| {
                pattern.len() <= design.len()
                    && is_possible(&design[pattern.len()..].to_vec(), available_patterns)
            })
    }
}

fn count_possibilities(
    design: &Vec<char>,
    available_patterns: &HashMap<char, Vec<Vec<char>>>,
    cache: &mut HashMap<Vec<char>, u64>,
) -> u64 {
    if design.len() == 0 {
        1
    } else {
        match cache.get(design) {
            Some(cached_value) => *cached_value,
            None => {
                let first_color = design[0];
                let result = available_patterns
                    .get(&first_color)
                    .unwrap_or(&vec![])
                    .iter()
                    .filter(|&pattern| {
                        pattern
                            .iter()
                            .zip(design)
                            .map(|pair| *pair.0 == *pair.1)
                            .all(|can_continue| can_continue)
                    })
                    .map(|pattern| {
                        if pattern.len() <= design.len() {
                            count_possibilities(
                                &design[pattern.len()..].to_vec(),
                                available_patterns,
                                cache,
                            )
                        } else {
                            0
                        }
                    })
                    .sum();
                cache.insert(design.clone(), result);
                result
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let available_patterns = parse_available_patterns(input);
    let wanted_designs = parse_wanted_designs(input);
    Some(
        wanted_designs
            .iter()
            .filter(|&design| is_possible(design, &available_patterns))
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let available_patterns = parse_available_patterns(input);
    let wanted_designs = parse_wanted_designs(input);
    let mut cache: HashMap<Vec<char>, u64> = HashMap::new();
    Some(
        wanted_designs
            .iter()
            .map(|design| count_possibilities(design, &available_patterns, &mut cache))
            .sum(),
    )
}

fn parse_available_patterns(input: &str) -> HashMap<char, Vec<Vec<char>>> {
    let mut available_patterns: HashMap<char, Vec<Vec<char>>> = HashMap::new();
    let patterns: Vec<&str> = input
        .lines()
        .take(1)
        .flat_map(|line| line.split(',').map(|pattern| pattern.trim()))
        .collect();
    for pattern in patterns {
        let pattern_as_chars = pattern.chars().collect::<Vec<char>>();
        let first_color = pattern_as_chars[0];
        match available_patterns.entry(first_color) {
            Entry::Occupied(mut occupied_entry) => {
                occupied_entry.get_mut().push(pattern_as_chars);
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(vec![pattern_as_chars]);
            }
        }
    }
    available_patterns
}

fn parse_wanted_designs(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .skip(2)
        .map(|line| line.chars().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
